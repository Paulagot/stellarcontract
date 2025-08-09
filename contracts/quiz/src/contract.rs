use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Address, BytesN, Env, Symbol, Vec, String,
    token::TokenClient,
};

// Hardcoded wallet addresses - using valid Stellar format
const PLATFORM_WALLET: &str = "GCBZQUCR63FVZY7PLWCIV5S4FOOK2KR2YRAHIFAY34MWLF5VYR7CJABV";
const CHARITY_WALLET: &str = "GCBZQUCR63FVZY7PLWCIV5S4FOOK2KR2YRAHIFAY34MWLF5VYR7CJABV";

// Optionally enforce allowed fee tokens in validate_fee_token() later
const _APPROVED_TOKENS: [&str; 1] = [
    "CDWKXA6BUAHIRNCYYC2NRYTRNASUMZTI3QYUTDPHOFUGXEWHUM4GYC35",
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
    pub contract_id: Address,
    pub amount: i128,
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
    host_fee_bps: u32,            // 0-500 (0-5%)
    prize_pool_bps: u32,          // 0-2500 (0-25%) - only used in PrizePoolSplit
    charity_bps: u32,             // 8000 - (host + prize_pool); must be >= 5000
    prize_mode: PrizeMode,
    prize_distribution: Vec<u32>, // Must sum to 100 for PrizePoolSplit
    prize_assets: Vec<Option<PrizeAsset>>, // For AssetBased [1st, 2nd, 3rd]
    ended: bool,                  // single phase flag now
    creation_ledger: u32,
    host_wallet: Option<Address>, // Required if host_fee_bps > 0
    players: Vec<PlayerEntry>,
    player_addresses: Vec<Address>,
    total_pool: i128,             // Total collected fees (entry + extras)
    total_entry_fees: i128,
    total_extras_fees: i128,
    total_paid_out: i128,
    winners: Vec<Address>,        // [1st, 2nd, 3rd] - set when game ends
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
}

#[contract]
pub struct QuizRoomContract;

#[contractimpl]
impl QuizRoomContract {
    // -----------------------
    // ROOM INITIALIZATION
    // -----------------------

    // Prize Pool Split room – players can join immediately
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

        // Basic amount guards
        if entry_fee <= 0 {
            return Err(QuizError::InsufficientPayment);
        }

        let storage_room_id = Self::u32_to_bytes(e, room_id);
        Self::validate_fee_token(e, &fee_token)?;

        let key = (Symbol::new(e, "config"), storage_room_id.clone());
        if e.storage().instance().has(&key) {
            return Err(QuizError::RoomAlreadyExists);
        }

        let host_fee_bps = host_fee_bps.unwrap_or(0);
        if host_fee_bps > 500 {
            return Err(QuizError::InvalidHostFee);
        }
        if prize_pool_bps > 2500 {
            return Err(QuizError::InvalidPrizePoolBps);
        }

        // host + prize_pool must be <= 8000 (80%)
        let total_allocated = host_fee_bps + prize_pool_bps;
        if total_allocated > 8000 {
            return Err(QuizError::InvalidTotalAllocation);
        }

        // Charity gets remaining up to 80% (>=50%)
        let charity_bps = 8000 - total_allocated;
        if charity_bps < 5000 {
            return Err(QuizError::CharityBelowMinimum);
        }

        // Build & validate prize distribution (first must be > 0 and sums to 100)
        if first_place_pct == 0 {
            return Err(QuizError::InvalidPrizeSplit);
        }
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
        if total_pct != 100 {
            return Err(QuizError::InvalidPrizeSplit);
        }

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

    // Asset-Based room – escrow 1..3 prize assets from host NOW
    // New signature keeps params <= 10 by bundling prizes into a Vec<PrizeAsset>.
    pub fn init_asset_room(
        e: &Env,
        room_id: u32,
        host: Address,
        fee_token: Address,
        entry_fee: i128,
        host_fee_bps: Option<u32>,
        prizes: Vec<PrizeAsset>,   // 1..=3 items
    ) -> Result<(), QuizError> {
        host.require_auth();

        // Amount guards
        if entry_fee <= 0 { return Err(QuizError::InsufficientPayment); }

        let n = prizes.len();
        if n == 0 || n > 3 { return Err(QuizError::InvalidPrizeAssets); }
        for i in 0..n {
            if let Some(p) = prizes.get(i) {
                if p.amount <= 0 { return Err(QuizError::InvalidPrizeAssets); }
            }
        }

        let storage_room_id = Self::u32_to_bytes(e, room_id);
        Self::validate_fee_token(e, &fee_token)?;

        let key = (Symbol::new(e, "config"), storage_room_id.clone());
        if e.storage().instance().has(&key) {
            return Err(QuizError::RoomAlreadyExists);
        }

        let host_fee_bps = host_fee_bps.unwrap_or(0);
        if host_fee_bps > 500 { return Err(QuizError::InvalidHostFee); }

        let charity_bps = 8000 - host_fee_bps;
        if charity_bps < 5000 { return Err(QuizError::CharityBelowMinimum); }

        // Escrow all provided prizes now
        let contract_address = e.current_contract_address();
        for i in 0..n {
            if let Some(p) = prizes.get(i) {
                Self::transfer_token(e, &p.contract_id, &host, &contract_address, p.amount)?;
            }
        }

        // Normalize to [Some(first), second?, third?] (fixed length 3)
        let p1 = prizes.get(0).map(|x| x);
        let p2 = prizes.get(1).map(|x| x);
        let p3 = prizes.get(2).map(|x| x);
        let prize_assets = Vec::from_array(e, [p1, p2, p3]);

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

        e.events().publish((
            Symbol::new(e, "asset_room_created"),
            room_id,
            host,
            entry_fee,
            host_fee_bps,
        ), ());

        Ok(())
    }

    // -----------------------
    // JOIN / PLAYERS
    // -----------------------

    // Player joins room and pays entry + extras
    pub fn join_room(
        e: &Env,
        room_id: u32,
        player: Address,
        screen_name: String,
        extras_amount: i128,
    ) -> Result<(), QuizError> {
        player.require_auth();

        if extras_amount < 0 {
            return Err(QuizError::InsufficientPayment);
        }

        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id.clone());

        let mut config: RoomConfig = e.storage().instance()
            .get(&key).ok_or(QuizError::RoomNotFound)?;

        if config.ended {
            return Err(QuizError::RoomAlreadyEnded);
        }

        // Validate screen name (basic)
        if screen_name.len() == 0 || screen_name.len() > 20 {
            return Err(QuizError::InvalidScreenName);
        }

        // Check duplicate player
        for i in 0..config.player_addresses.len() {
            if let Some(existing) = config.player_addresses.get(i) {
                if existing == player {
                    return Err(QuizError::PlayerAlreadyJoined);
                }
            }
        }

        // Check screen name taken
        for i in 0..config.players.len() {
            if let Some(entry) = config.players.get(i) {
                if entry.screen_name == screen_name {
                    return Err(QuizError::ScreenNameTaken);
                }
            }
        }

        // Transfer entry + extras from player to contract
        let total_payment = config.entry_fee + extras_amount;
        if total_payment <= 0 {
            return Err(QuizError::InsufficientPayment);
        }
        let contract_address = e.current_contract_address();
        Self::transfer_token(e, &config.fee_token, &player, &contract_address, total_payment)?;

        // Record
        let entry = PlayerEntry {
            player: player.clone(),
            screen_name: screen_name.clone(),
            entry_paid: config.entry_fee,
            extras_paid: extras_amount,
            total_paid: total_payment,
            join_ledger: e.ledger().sequence(),
        };

        config.players.push_back(entry);
        config.player_addresses.push_back(player.clone());
        config.total_pool += total_payment;
        config.total_entry_fees += config.entry_fee;
        config.total_extras_fees += extras_amount;

        e.storage().instance().set(&key, &config);

        e.events().publish((
            Symbol::new(e, "player_joined"),
            room_id,
            player,
            screen_name,
            total_payment
        ),());

        Ok(())
    }

    // -----------------------
    // END / PAYOUTS
    // -----------------------

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
            .get(&key).ok_or(QuizError::RoomNotFound)?;

        // Only host
        config.host.require_auth();

        if config.ended {
            return Err(QuizError::RoomAlreadyEnded);
        }

        // Must have at least one player
        if config.players.len() == 0 {
            return Err(QuizError::InsufficientPlayers);
        }

        // Build winners list
        let mut winners = Vec::new(e);
        if let Some(w) = first_place { winners.push_back(w); }
        if let Some(w) = second_place { winners.push_back(w); }
        if let Some(w) = third_place { winners.push_back(w); }

        // Validate winners: must be players + unique
        Self::validate_winners(e, &config, &winners)?;

        config.winners = winners;
        config.ended = true;

        e.storage().instance().set(&key, &config);

        // Distribute prizes
        Self::distribute_prizes_internal(e, &config)?;

        e.events().publish((
            Symbol::new(e, "game_ended"),
            room_id,
            config.winners.len(),
            config.total_pool
        ),());

        Ok(())
    }

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
            .get(&key).ok_or(QuizError::RoomNotFound)?;

        // Only host
        config.host.require_auth();

        if config.ended {
            return Err(QuizError::RoomAlreadyEnded);
        }

        if config.players.len() == 0 {
            return Err(QuizError::InsufficientPlayers);
        }

        let mut winners = Vec::new(e);
        if let Some(name) = first_place_name {
            if let Some(addr) = Self::get_player_by_screen_name(e, room_id, name) { winners.push_back(addr); }
        }
        if let Some(name) = second_place_name {
            if let Some(addr) = Self::get_player_by_screen_name(e, room_id, name) { winners.push_back(addr); }
        }
        if let Some(name) = third_place_name {
            if let Some(addr) = Self::get_player_by_screen_name(e, room_id, name) { winners.push_back(addr); }
        }

        // Validate winners
        Self::validate_winners(e, &config, &winners)?;

        config.winners = winners;
        config.ended = true;

        e.storage().instance().set(&key, &config);

        Self::distribute_prizes_internal(e, &config)?;

        e.events().publish((
            Symbol::new(e, "game_ended"),
            room_id,
            config.winners.len(),
            config.total_pool
        ),());

        Ok(())
    }

    // -----------------------
    // QUERIES
    // -----------------------

    pub fn get_room_players(e: &Env, room_id: u32) -> Vec<PlayerEntry> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        if let Some(cfg) = e.storage().instance().get::<_, RoomConfig>(&key) {
            cfg.players
        } else {
            Vec::new(e)
        }
    }

    pub fn get_player_by_screen_name(e: &Env, room_id: u32, screen_name: String) -> Option<Address> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        if let Some(cfg) = e.storage().instance().get::<_, RoomConfig>(&key) {
            for i in 0..cfg.players.len() {
                if let Some(pe) = cfg.players.get(i) {
                    if pe.screen_name == screen_name {
                        return Some(pe.player);
                    }
                }
            }
        }
        None
    }

    pub fn get_room_config(e: &Env, room_id: u32) -> Option<RoomConfig> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);
        e.storage().instance().get(&key)
    }

    pub fn get_room_financials(e: &Env, room_id: u32) -> Option<(i128, i128, i128, i128, i128)> {
        let storage_room_id = Self::u32_to_bytes(e, room_id);
        let key = (Symbol::new(e, "config"), storage_room_id);

        if let Some(cfg) = e.storage().instance().get::<_, RoomConfig>(&key) {
            let platform_amount = (cfg.total_pool * 2000) / 10000; // 20%
            let charity_amount  = (cfg.total_pool * cfg.charity_bps as i128) / 10000;
            let host_amount     = (cfg.total_pool * cfg.host_fee_bps as i128) / 10000;
            let prize_amount    = cfg.total_pool - platform_amount - charity_amount - host_amount;
            let total_should_pay = platform_amount + charity_amount + host_amount + prize_amount;

            Some((
                cfg.total_pool,                 // total collected
                cfg.total_entry_fees,
                cfg.total_extras_fees,
                total_should_pay,               // expected payouts
                cfg.total_pool - total_should_pay
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

    // -----------------------
    // INTERNALS
    // -----------------------

    fn validate_winners(e: &Env, cfg: &RoomConfig, winners: &Vec<Address>) -> Result<(), QuizError> {
        // ensure every winner is a player and no duplicates
        let mut seen = Vec::new(e);
        for i in 0..winners.len() {
            if let Some(w) = winners.get(i) {
                if !Self::ensure_is_player(cfg, &w) { return Err(QuizError::InvalidWinners); }
                for j in 0..seen.len() {
                    if let Some(x) = seen.get(j) { if x == w { return Err(QuizError::InvalidWinners); } }
                }
                seen.push_back(w);
            }
        }
        Ok(())
    }

    fn ensure_is_player(cfg: &RoomConfig, addr: &Address) -> bool {
        for i in 0..cfg.player_addresses.len() {
            if let Some(a) = cfg.player_addresses.get(i) {
                if &a == addr { return true; }
            }
        }
        false
    }

    fn distribute_prizes_internal(e: &Env, config: &RoomConfig) -> Result<(), QuizError> {
        let contract_address = e.current_contract_address();
        let platform_address = Address::from_string(&String::from_str(e, PLATFORM_WALLET));
        let charity_address  = Address::from_string(&String::from_str(e, CHARITY_WALLET));

        let platform_amount = (config.total_pool * 2000) / 10000; // 20%
        let charity_amount  = (config.total_pool * config.charity_bps as i128) / 10000;
        let host_amount     = (config.total_pool * config.host_fee_bps as i128) / 10000;
        let prize_amount    = config.total_pool - platform_amount - charity_amount - host_amount;

        let mut total_distributed = 0i128;

        // platform
        Self::transfer_token(e, &config.fee_token, &contract_address, &platform_address, platform_amount)?;
        total_distributed += platform_amount;

        // charity
        Self::transfer_token(e, &config.fee_token, &contract_address, &charity_address, charity_amount)?;
        total_distributed += charity_amount;

        // host
        if host_amount > 0 {
            if let Some(host_wallet) = &config.host_wallet {
                Self::transfer_token(e, &config.fee_token, &contract_address, host_wallet, host_amount)?;
                total_distributed += host_amount;
            }
        }

        match config.prize_mode {
            PrizeMode::PrizePoolSplit => {
                for i in 0..config.winners.len().min(config.prize_distribution.len()) {
                    if let (Some(winner), Some(pct)) = (config.winners.get(i), config.prize_distribution.get(i)) {
                        let amt = (prize_amount * pct as i128) / 100;
                        if amt > 0 {
                            Self::transfer_token(e, &config.fee_token, &contract_address, &winner, amt)?;
                            total_distributed += amt;
                        }
                    }
                }
            }
            PrizeMode::AssetBased => {
                for i in 0..config.winners.len().min(3) {
                    if let (Some(winner), Some(Some(prize_asset))) = (config.winners.get(i), config.prize_assets.get(i)) {
                        Self::transfer_token(e, &prize_asset.contract_id, &contract_address, &winner, prize_asset.amount)?;
                        // asset prizes don't count toward total_distributed (different token)
                    }
                }
            }
        }

        // send any remainder (dust) to charity so contract doesn't trap funds
        let remainder = config.total_pool - total_distributed;
        if remainder > 0 {
            Self::transfer_token(e, &config.fee_token, &contract_address, &charity_address, remainder)?;
        }

        e.events().publish((
            Symbol::new(e, "prizes_distributed"),
            config.room_id.clone(),
            platform_amount,
            charity_amount,
            host_amount,
            prize_amount,
            total_distributed
        ),());

        Ok(())
    }

    fn u32_to_bytes(e: &Env, value: u32) -> BytesN<32> {
        let mut bytes = [0u8; 32];
        let value_bytes = value.to_be_bytes();
        bytes[28..32].copy_from_slice(&value_bytes);
        BytesN::from_array(e, &bytes)
    }

    fn validate_fee_token(_e: &Env, _token: &Address) -> Result<(), QuizError> {
        // For testing, accept any address.
        // TODO (mainnet/testnet): enforce an allowlist or check token metadata here.
        Ok(())
    }

    fn transfer_token(
        e: &Env,
        token: &Address,
        from: &Address,
        to: &Address,
        amount: i128,
    ) -> Result<(), QuizError> {
        let token_client = TokenClient::new(e, token);
        token_client.transfer(from, to, &amount);
        Ok(())
    }
}



