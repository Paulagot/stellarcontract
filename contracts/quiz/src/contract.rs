use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Address, BytesN, Env, Symbol, Vec, String,
    token::TokenClient,
};

// Hardcoded wallet addresses - using valid Stellar format
const PLATFORM_WALLET: &str = "GCBZQUCR63FVZY7PLWCIV5S4FOOK2KR2YRAHIFAY34MWLF5VYR7CJABV";
const CHARITY_WALLET: &str = "GCBZQUCR63FVZY7PLWCIV5S4FOOK2KR2YRAHIFAY34MWLF5VYR7CJABV";

// Approved fee tokens for testnet
const APPROVED_TOKENS: [&str; 2] = [
    "CC4ISY3QVTU3KORGOYODCY3PR274TE2IBU6XFUMIVYU7KT7DRRAST2MM", // Mock USDC
    "CC4ISY3QVTU3KORGOYODCY3PR274TE2IBU6XFUMIVYU7KT7DRRAST2MM", // Mock GLO
];

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum PrizeMode {
    PrizePoolSplit,
    AssetBased,
}

#[derive(Clone)]
#[contracttype]
pub struct PrizeAsset {
    contract_id: Address,
    amount: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct PlayerEntry {
    player: Address,
    screen_name: String,
    entry_paid: i128,
    extras_paid: i128,
    total_paid: i128,
    join_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct RoomConfig {
    room_id: BytesN<32>,
    host: Address,
    fee_token: Address,
    entry_fee: i128,
    host_fee_bps: u32,          // 0-500 (0-5%)
    prize_pool_bps: u32,        // 0-2500 (0-25%) - only used in PrizePoolSplit
    charity_bps: u32,           // Calculated based on other fees
    prize_mode: PrizeMode,
    prize_distribution: Vec<u32>, // Must sum to 100, for PrizePoolSplit
    prize_assets: Vec<Option<PrizeAsset>>, // For AssetBased [1st, 2nd, 3rd]
    started: bool,
    game_started: bool,          // Tracks when quiz questions actually begin
    ended: bool,
    creation_ledger: u32,
    host_wallet: Option<Address>, // Required if host_fee_bps > 0
    players: Vec<PlayerEntry>,       // Store PlayerEntry instead of just Address
    player_addresses: Vec<Address>,  // Keep addresses separate for easy lookup
    total_pool: i128,            // Total collected fees
    total_entry_fees: i128,      // Total entry fees collected
    total_extras_fees: i128,     // Total extras fees collected
    total_paid_out: i128,        // Total distributed (should equal total_pool when game ends)
    winners: Vec<Address>,       // [1st, 2nd, 3rd] - set when game ends
}

#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuizError {
    InvalidHostFee = 1,
    MissingHostWallet = 2,
    InvalidPrizeSplit = 3,
    CharityBelowMinimum = 4,
    InvalidPrizePoolBps = 5,
    MissingPrizePoolConfig = 6,
    MissingPrizeAssets = 7,
    InvalidPrizeAssets = 8,
    InvalidTotalAllocation = 9,
    InvalidFeeToken = 10,
    RoomAlreadyExists = 11,
    RoomNotFound = 12,
    RoomAlreadyStarted = 13,
    RoomNotStarted = 14,
    RoomAlreadyEnded = 15,
    PlayerAlreadyJoined = 16,
    InsufficientPayment = 17,
    Unauthorized = 18,
    InvalidWinners = 19,
    AssetTransferFailed = 20,
    InsufficientPlayers = 21,
    InsufficientAssets = 22,
    DepositFailed = 23,
    ScreenNameTaken = 24,
    InvalidScreenName = 25,
    GameAlreadyStarted = 26,
}

#[contract]
pub struct QuizRoomContract;

#[contractimpl]
impl QuizRoomContract {
    // Create Prize Pool Split room - all entry fees go to prize pool
    pub fn init_pool_room(
        e: &Env,
        room_id: u32,
        host: Address,
        fee_token: Address,
        entry_fee: i128,
        host_fee_bps: Option<u32>,
        prize_pool_bps: u32,
        first_place_pct: u32,
        second_place_pct: Option<u32>,
        third_place_pct: Option<u32>,
    ) -> Result<(), QuizError> {
        host.require_auth();

        let storage_room_id = Self::u32_to_bytes(e, room_id);
        Self::validate_fee_token(e, &fee_token)?;

        // Check if room already exists
        let key = (Symbol::new(e, "config"), storage_room_id.clone());
        if e.storage().instance().has(&key) {
            return Err(QuizError::RoomAlreadyExists);
        }

        // Set defaults
        let host_fee_bps = host_fee_bps.unwrap_or(0);

        // Validate host fee (0-5%)
        if host_fee_bps > 500 {
            return Err(QuizError::InvalidHostFee);
        }

        // Validate prize pool (0-25%)
        if prize_pool_bps > 2500 {
            return Err(QuizError::InvalidPrizePoolBps);
        }

        // Check total allocation doesn't exceed 80%
        let total_allocated = host_fee_bps + prize_pool_bps;
        if total_allocated > 8000 {
            return Err(QuizError::InvalidTotalAllocation);
        }

        // Calculate charity percentage (80% - host_fee - prize_pool)
        let charity_bps = 8000 - total_allocated;
        
        // Ensure charity gets at least 50% of total
        if charity_bps < 5000 {
            return Err(QuizError::CharityBelowMinimum);
        }

        // Build prize distribution
        let mut distribution = Vec::new(e);
        let mut total_pct = first_place_pct;
        
        distribution.push_back(first_place_pct);
        
        if let Some(second_pct) = second_place_pct {
            if second_pct > 0 {
                distribution.push_back(second_pct);
                total_pct += second_pct;
            }
        }
        
        if let Some(third_pct) = third_place_pct {
            if third_pct > 0 {
                distribution.push_back(third_pct);
                total_pct += third_pct;
            }
        }

        // Validate percentages sum to 100
        if total_pct != 100 {
            return Err(QuizError::InvalidPrizeSplit);
        }

        // Create room config
        let config = RoomConfig {
            room_id: storage_room_id.clone(),
            host: host.clone(),
            fee_token: fee_token.clone(),
            entry_fee,
            host_fee_bps,
            prize_pool_bps,
            charity_bps,
            prize_mode: PrizeMode::PrizePoolSplit,
            prize_distribution: distribution,
            prize_assets: Vec::from_array(e, [None, None, None]),
            started: false,
            game_started: false,
            ended: false,
            creation_ledger: e.ledger().sequence(),
            host_wallet: Some(host.clone()),
            players: Vec::new(e),
            player_addresses: Vec::new(e),
            total_pool: 0,
            total_entry_fees: 0,
            total_extras_fees: 0,
            total_paid_out: 0,
            winners: Vec::new(e),
        };

        e.storage().instance().set(&key, &config);

        // Emit room creation event
        e.events().publish((
            Symbol::new(e, "pool_room_created"),
            room_id,
            host,
            entry_fee,
            host_fee_bps,
            prize_pool_bps
        ), ());

        Ok(())
    }

    // Create Asset Based room - host provides specific prize assets
    pub fn init_asset_room(
        e: &Env,
        room_id: u32,
        host: Address,
        fee_token: Address,
        entry_fee: i128,
        host_fee_bps: Option<u32>,
        first_prize_token: Address,
        first_prize_amount: i128,
        second_prize_token: Option<Address>,
        second_prize_amount: Option<i128>,
    ) -> Result<(), QuizError> {
        host.require_auth();

        let storage_room_id = Self::u32_to_bytes(e, room_id);
        Self::validate_fee_token(e, &fee_token)?;

        // Check if room already exists
        let key = (Symbol::new(e, "config"), storage_room_id.clone());
        if e.storage().instance().has(&key) {
            return Err(QuizError::RoomAlreadyExists);
        }

        // Set defaults
        let host_fee_bps = host_fee_bps.unwrap_or(0);

        // Validate host fee (0-5%)
        if host_fee_bps > 500 {
            return Err(QuizError::InvalidHostFee);
        }

        // For asset mode: charity gets 80% - host_fee
        let charity_bps = 8000 - host_fee_bps;
        
        // Ensure charity gets at least 50%
        if charity_bps < 5000 {
            return Err(QuizError::CharityBelowMinimum);
        }

        // Build prize assets (first and second only)
        let first_prize = PrizeAsset {
            contract_id: first_prize_token,
            amount: first_prize_amount,
        };

        let second_prize = if let (Some(token), Some(amount)) = (second_prize_token, second_prize_amount) {
            Some(PrizeAsset {
                contract_id: token,
                amount,
            })
        } else {
            None
        };

        let prize_assets = Vec::from_array(e, [Some(first_prize), second_prize, None]);

        // Create room config
        let config = RoomConfig {
            room_id: storage_room_id.clone(),
            host: host.clone(),
            fee_token: fee_token.clone(),
            entry_fee,
            host_fee_bps,
            prize_pool_bps: 0,
            charity_bps,
            prize_mode: PrizeMode::AssetBased,
            prize_distribution: Vec::new(e),
            prize_assets,
            started: false,
            game_started: false,
            ended: false,
            creation_ledger: e.ledger().sequence(),
            host_wallet: Some(host.clone()),
            players: Vec::new(e),
            player_addresses: Vec::new(e),
            total_pool: 0,
            total_entry_fees: 0,
            total_extras_fees: 0,
            total_paid_out: 0,
            winners: Vec::new(e),
        };

        e.storage().instance().set(&key, &config);

        // Emit room creation event
        e.events().publish((
            Symbol::new(e, "asset_room_created"),
            room_id,
            host,
            entry_fee,
            host_fee_bps,
            first_prize_amount
        ), ());

        Ok(())
    }

    // Add third place prize to existing asset room (must be called BEFORE start_room)
    pub fn add_third_prize(
        e: &Env,
        room_id: u32,
        host: Address,
        third_prize_token: Address,
        third_prize_amount: i128,
    ) -> Result<(), QuizError> {
        host.require_auth();

        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        
        let mut config: RoomConfig = e.storage().instance()
            .get(&key)
            .ok_or(QuizError::RoomNotFound)?;

        // Can only modify before game starts
        if config.started {
            return Err(QuizError::RoomAlreadyStarted);
        }

        // Only for asset-based rooms
        if config.prize_mode != PrizeMode::AssetBased {
            return Err(QuizError::InvalidPrizeAssets);
        }

        // Add third prize
        let third_prize = PrizeAsset {
            contract_id: third_prize_token,
            amount: third_prize_amount,
        };

        // Update the third position in prize_assets
        let mut updated_assets = Vec::new(e);
        updated_assets.push_back(config.prize_assets.get(0).unwrap()); // First prize
        updated_assets.push_back(config.prize_assets.get(1).unwrap()); // Second prize (may be None)
        updated_assets.push_back(Some(third_prize)); // Third prize

        config.prize_assets = updated_assets;
        e.storage().instance().set(&key, &config);
        Ok(())
    }

    // Add sponsor deposit function - anyone can contribute assets to a room
    pub fn deposit_prize_assets(
        e: &Env,
        room_id: u32,
        depositor: Address,
        asset_token: Address,
        amount: i128,
    ) -> Result<(), QuizError> {
        depositor.require_auth();

        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id.clone());
        
        let config: RoomConfig = e.storage().instance()
            .get(&key)
            .ok_or(QuizError::RoomNotFound)?;

        // Can only deposit before game starts
        if config.started {
            return Err(QuizError::RoomAlreadyStarted);
        }

        // Only for asset-based rooms
        if config.prize_mode != PrizeMode::AssetBased {
            return Err(QuizError::InvalidPrizeAssets);
        }

        // Transfer assets from depositor to contract
        let contract_address = e.current_contract_address();
        Self::transfer_token(e, &asset_token, &depositor, &contract_address, amount)?;

        // Track the deposit
        let deposit_key = (Symbol::new(e, "deposit"), storage_room_id, asset_token);
        let current_balance: i128 = e.storage().instance().get(&deposit_key).unwrap_or(0);
        e.storage().instance().set(&deposit_key, &(current_balance + amount));

        Ok(())
    }

    // Enhanced start_room with asset validation
    pub fn start_room(e: &Env, room_id: u32) -> Result<(), QuizError> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id.clone());
        
        let mut config: RoomConfig = e.storage().instance()
            .get(&key)
            .ok_or(QuizError::RoomNotFound)?;

        // Only host can start
        config.host.require_auth();

        if config.started {
            return Err(QuizError::RoomAlreadyStarted);
        }

        // For asset-based rooms: Validate all required assets are deposited
        if config.prize_mode == PrizeMode::AssetBased {
            for i in 0..config.prize_assets.len() {
                if let Some(Some(prize_asset)) = config.prize_assets.get(i) {
                    let deposit_key = (Symbol::new(e, "deposit"), storage_room_id.clone(), prize_asset.contract_id.clone());
                    let deposited_amount: i128 = e.storage().instance().get(&deposit_key).unwrap_or(0);
                    
                    if deposited_amount < prize_asset.amount {
                        return Err(QuizError::InsufficientAssets);
                    }
                }
            }
        }

        config.started = true;
        e.storage().instance().set(&key, &config);

        // Emit event for frontend
        e.events().publish((
            Symbol::new(e, "room_started"),
            room_id,
            config.host
        ), ());

        Ok(())
    }

    // Host starts the actual quiz (no more players can join after this)
    pub fn start_quiz(e: &Env, room_id: u32) -> Result<(), QuizError> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        
        let mut config: RoomConfig = e.storage().instance()
            .get(&key)
            .ok_or(QuizError::RoomNotFound)?;

        // Only host can start quiz
        config.host.require_auth();

        if !config.started {
            return Err(QuizError::RoomNotStarted);
        }

        if config.game_started {
            return Err(QuizError::GameAlreadyStarted);
        }

        if config.ended {
            return Err(QuizError::RoomAlreadyEnded);
        }

        // Must have at least 1 player to start quiz
        if config.players.len() == 0 {
            return Err(QuizError::InsufficientPlayers);
        }

        config.game_started = true;
        e.storage().instance().set(&key, &config);

        // Emit event for frontend
        e.events().publish((
            Symbol::new(e, "quiz_started"),
            room_id,
            config.players.len()
        ), ());

        Ok(())
    }

    // Check current asset deposit status for a room
    pub fn get_asset_deposit_status(e: &Env, room_id: u32) -> Vec<(Address, i128, i128)> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id.clone());
        
        let mut status = Vec::new(e);
        
        if let Some(config) = e.storage().instance().get::<_, RoomConfig>(&key) {
            if config.prize_mode == PrizeMode::AssetBased {
                for i in 0..config.prize_assets.len() {
                    if let Some(Some(prize_asset)) = config.prize_assets.get(i) {
                        let deposit_key = (Symbol::new(e, "deposit"), storage_room_id.clone(), prize_asset.contract_id.clone());
                        let deposited: i128 = e.storage().instance().get(&deposit_key).unwrap_or(0);
                        
                        // Return (token_address, required_amount, deposited_amount)
                        status.push_back((prize_asset.contract_id, prize_asset.amount, deposited));
                    }
                }
            }
        }
        
        status
    }

    // Allow sponsors to be listed (optional feature)
    pub fn add_sponsor(
        e: &Env,
        room_id: u32,
        host: Address,
        sponsor: Address,
    ) -> Result<(), QuizError> {
        host.require_auth();

        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let sponsors_key = (Symbol::new(e, "sponsors"), storage_room_id);
        
        let mut sponsors: Vec<Address> = e.storage().instance().get(&sponsors_key).unwrap_or(Vec::new(e));
        sponsors.push_back(sponsor);
        e.storage().instance().set(&sponsors_key, &sponsors);
        
        Ok(())
    }

    // Get list of sponsors for a room
    pub fn get_sponsors(e: &Env, room_id: u32) -> Vec<Address> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let sponsors_key = (Symbol::new(e, "sponsors"), storage_room_id);
        e.storage().instance().get(&sponsors_key).unwrap_or(Vec::new(e))
    }

    // Player joins room with screen name and pays entry + extras fees
    pub fn join_room(
        e: &Env, 
        room_id: u32, 
        player: Address, 
        screen_name: String,
        extras_amount: i128
    ) -> Result<(), QuizError> {
        player.require_auth();

        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id.clone());
        
        let mut config: RoomConfig = e.storage().instance()
            .get(&key)
            .ok_or(QuizError::RoomNotFound)?;

        if !config.started {
            return Err(QuizError::RoomNotStarted);
        }

        // Players cannot join once quiz questions have started
        if config.game_started {
            return Err(QuizError::GameAlreadyStarted);
        }

        if config.ended {
            return Err(QuizError::RoomAlreadyEnded);
        }

        // Validate screen name
        if screen_name.len() == 0 || screen_name.len() > 20 {
            return Err(QuizError::InvalidScreenName);
        }

        // Check if player already joined
        for i in 0..config.player_addresses.len() {
            if let Some(existing_player) = config.player_addresses.get(i) {
                if existing_player == player {
                    return Err(QuizError::PlayerAlreadyJoined);
                }
            }
        }

        // Check if screen name is taken
        for i in 0..config.players.len() {
            if let Some(existing_entry) = config.players.get(i) {
                if existing_entry.screen_name == screen_name {
                    return Err(QuizError::ScreenNameTaken);
                }
            }
        }

        // Calculate total payment (entry fee + extras)
        let entry_fee = config.entry_fee;
        let total_payment = entry_fee + extras_amount;

        // Transfer tokens from player to contract
        let contract_address = e.current_contract_address();
        Self::transfer_token(e, &config.fee_token, &player, &contract_address, total_payment)?;

        // Create player entry
        let player_entry = PlayerEntry {
            player: player.clone(),
            screen_name: screen_name.clone(),
            entry_paid: entry_fee,
            extras_paid: extras_amount,
            total_paid: total_payment,
            join_ledger: e.ledger().sequence(),
        };

        // Add player to both lists
        config.players.push_back(player_entry);
        config.player_addresses.push_back(player.clone());
        config.total_pool += total_payment;
        config.total_entry_fees += entry_fee;
        config.total_extras_fees += extras_amount;

        e.storage().instance().set(&key, &config);

        // Emit success event
        e.events().publish((
            Symbol::new(e, "player_joined"),
            room_id,
            player,
            screen_name,
            total_payment
        ), ());

        Ok(())
    }

    // End game and set winners
    pub fn end_room(
        e: &Env,
        room_id: u32,
        first_place: Option<Address>,
        second_place: Option<Address>,
        third_place: Option<Address>,
    ) -> Result<(), QuizError> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        
        let mut config: RoomConfig = e.storage().instance()
            .get(&key)
            .ok_or(QuizError::RoomNotFound)?;

        // Only host can end game
        config.host.require_auth();

        if !config.started {
            return Err(QuizError::RoomNotStarted);
        }

        // Can only end if quiz has actually started
        if !config.game_started {
            return Err(QuizError::RoomNotStarted);
        }

        if config.ended {
            return Err(QuizError::RoomAlreadyEnded);
        }

        // Set winners
        let mut winners = Vec::new(e);
        if let Some(winner) = first_place {
            winners.push_back(winner);
        }
        if let Some(winner) = second_place {
            winners.push_back(winner);
        }
        if let Some(winner) = third_place {
            winners.push_back(winner);
        }

        config.winners = winners;
        config.ended = true;

        e.storage().instance().set(&key, &config);

        // Automatically distribute prizes
        Self::distribute_prizes_internal(e, &config)?;

        // Emit game ended event
        e.events().publish((
            Symbol::new(e, "game_ended"),
            room_id,
            config.winners.len(),
            config.total_pool
        ), ());

        Ok(())
    }

    // Get list of players with their screen names and payments
    pub fn get_room_players(e: &Env, room_id: u32) -> Vec<PlayerEntry> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        if let Some(config) = e.storage().instance().get::<_, RoomConfig>(&key) {
            config.players
        } else {
            Vec::new(e)
        }
    }

    // Get player by screen name (for winner selection)
    pub fn get_player_by_screen_name(e: &Env, room_id: u32, screen_name: String) -> Option<Address> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        
        if let Some(config) = e.storage().instance().get::<_, RoomConfig>(&key) {
            for i in 0..config.players.len() {
                if let Some(player_entry) = config.players.get(i) {
                    if player_entry.screen_name == screen_name {
                        return Some(player_entry.player);
                    }
                }
            }
        }
        None
    }

    // New function for host to end game using screen names
    pub fn end_room_by_screen_names(
        e: &Env,
        room_id: u32,
        first_place_name: Option<String>,
        second_place_name: Option<String>,
        third_place_name: Option<String>,
    ) -> Result<(), QuizError> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        
        let mut config: RoomConfig = e.storage().instance()
            .get(&key)
            .ok_or(QuizError::RoomNotFound)?;

        // Only host can end game
        config.host.require_auth();

        if !config.started {
            return Err(QuizError::RoomNotStarted);
        }

        // Can only end if quiz has actually started
        if !config.game_started {
            return Err(QuizError::RoomNotStarted);
        }

        if config.ended {
            return Err(QuizError::RoomAlreadyEnded);
        }

        // Convert screen names to addresses
        let mut winners = Vec::new(e);
        
        if let Some(name) = first_place_name {
            if let Some(address) = Self::get_player_by_screen_name(e, room_id, name) {
                winners.push_back(address);
            }
        }
        
        if let Some(name) = second_place_name {
            if let Some(address) = Self::get_player_by_screen_name(e, room_id, name) {
                winners.push_back(address);
            }
        }
        
        if let Some(name) = third_place_name {
            if let Some(address) = Self::get_player_by_screen_name(e, room_id, name) {
                winners.push_back(address);
            }
        }

        config.winners = winners;
        config.ended = true;

        e.storage().instance().set(&key, &config);

        // Automatically distribute prizes
        Self::distribute_prizes_internal(e, &config)?;

        // Emit game ended event
        e.events().publish((
            Symbol::new(e, "game_ended"),
            room_id,
            config.winners.len(),
            config.total_pool
        ), ());

        Ok(())
    }

    // Internal prize distribution
    fn distribute_prizes_internal(e: &Env, config: &RoomConfig) -> Result<(), QuizError> {
        let contract_address = e.current_contract_address();
        let platform_address = Address::from_string(&String::from_str(e, PLATFORM_WALLET));
        let charity_address = Address::from_string(&String::from_str(e, CHARITY_WALLET));

        // Calculate distributions from total pool
        let platform_amount = (config.total_pool * 2000) / 10000; // 20%
        let charity_amount = (config.total_pool * config.charity_bps as i128) / 10000;
        let host_amount = (config.total_pool * config.host_fee_bps as i128) / 10000;
        let prize_amount = config.total_pool - platform_amount - charity_amount - host_amount;

        let mut total_distributed = 0i128;

        // Transfer platform fee
        Self::transfer_token(e, &config.fee_token, &contract_address, &platform_address, platform_amount)?;
        total_distributed += platform_amount;

        // Transfer charity amount
        Self::transfer_token(e, &config.fee_token, &contract_address, &charity_address, charity_amount)?;
        total_distributed += charity_amount;

        // Transfer host fee (if any)
        if host_amount > 0 {
            if let Some(host_wallet) = &config.host_wallet {
                Self::transfer_token(e, &config.fee_token, &contract_address, host_wallet, host_amount)?;
                total_distributed += host_amount;
            }
        }

        // Distribute prizes based on mode
        match config.prize_mode {
            PrizeMode::PrizePoolSplit => {
                // Distribute prize pool to winners based on percentages
                for i in 0..config.winners.len().min(config.prize_distribution.len()) {
                    if let (Some(winner), Some(percentage)) = (config.winners.get(i), config.prize_distribution.get(i)) {
                        let winner_amount = (prize_amount * percentage as i128) / 100;
                        Self::transfer_token(e, &config.fee_token, &contract_address, &winner, winner_amount)?;
                        total_distributed += winner_amount;
                    }
                }
            },
            PrizeMode::AssetBased => {
                // Transfer specific assets to winners
                for i in 0..config.winners.len().min(3) {
                    if let (Some(winner), Some(Some(prize_asset))) = (config.winners.get(i), config.prize_assets.get(i)) {
                        Self::transfer_token(e, &prize_asset.contract_id, &contract_address, &winner, prize_asset.amount)?;
                        // Note: Asset prizes don't count toward total_distributed since they're separate tokens
                    }
                }
            }
        }

        // Emit distribution summary event
        e.events().publish((
            Symbol::new(e, "prizes_distributed"),
            config.room_id.clone(),
            platform_amount,
            charity_amount,
            host_amount,
            prize_amount,
            total_distributed
        ), ());

        Ok(())
    }

    // Helper functions
    fn u32_to_bytes(e: &Env, value: u32) -> BytesN<32> {
        let mut bytes = [0u8; 32];
        let value_bytes = value.to_be_bytes();
        bytes[28..32].copy_from_slice(&value_bytes);
        BytesN::from_array(e, &bytes)
    }

    fn validate_fee_token(_e: &Env, _token: &Address) -> Result<(), QuizError> {
        // For testing, accept any valid address format
        // In production, you'd check against approved token list
        Ok(())
    }

    fn transfer_token(
        e: &Env,
        token: &Address,
        from: &Address,
        to: &Address,
        amount: i128,
    ) -> Result<(), QuizError> {
        // Real Stellar token transfer
        let token_client = TokenClient::new(e, token);
        token_client.transfer(from, to, &amount);
        Ok(())
    }

    // Query functions
    pub fn get_room_config(e: &Env, room_id: u32) -> Option<RoomConfig> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        e.storage().instance().get(&key)
    }

    // Get detailed financial summary
    pub fn get_room_financials(e: &Env, room_id: u32) -> Option<(i128, i128, i128, i128, i128)> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        
        if let Some(config) = e.storage().instance().get::<_, RoomConfig>(&key) {
            // Calculate what should be paid out
            let platform_amount = (config.total_pool * 2000) / 10000; // 20%
            let charity_amount = (config.total_pool * config.charity_bps as i128) / 10000;
            let host_amount = (config.total_pool * config.host_fee_bps as i128) / 10000;
            let prize_amount = config.total_pool - platform_amount - charity_amount - host_amount;
            let total_should_pay = platform_amount + charity_amount + host_amount + prize_amount;
            
            // Return: (total_collected, entry_fees, extras_fees, total_should_pay_out, remaining_balance)
            Some((
                config.total_pool,                    // Total collected
                config.total_entry_fees,              // Entry fees only
                config.total_extras_fees,             // Extras fees only
                total_should_pay,                     // What should be paid out
                config.total_pool - total_should_pay  // Remaining (should be 0)
            ))
        } else {
            None
        }
    }

    pub fn get_platform_wallet(e: &Env) -> String {
        String::from_str(e, PLATFORM_WALLET)
    }

    pub fn get_charity_wallet(e: &Env) -> String {
        String::from_str(e, CHARITY_WALLET)
    }
}
