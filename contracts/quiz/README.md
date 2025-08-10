# Soroban Quiz Room Contract Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Security Features](#security-features)
4. [Deployment Guide](#deployment-guide)
5. [API Reference](#api-reference)
6. [Usage Examples](#usage-examples)
7. [Error Handling](#error-handling)
8. [Frontend Integration](#frontend-integration)
9. [Testing Guide](#testing-guide)
10. [Maintenance & Operations](#maintenance--operations)

---

## Overview

The Soroban Quiz Room Contract is a secure, feature-rich smart contract for managing quiz competitions on the Stellar blockchain. It supports multiple prize distribution modes, comprehensive fee management, and robust security measures.

### Key Features
- **Two Prize Modes**: Prize Pool Split and Asset-Based prizes
- **Multi-Stakeholder Fee Distribution**: Platform, charity, host, and winners
- **Token Allowlist System**: Admin-controlled approved tokens
- **Emergency Controls**: Pause functionality and admin management
- **Gas Optimized**: O(1) player lookups and efficient storage
- **Security Hardened**: Protection against overflow, reentrancy, and other attacks

### Stakeholders
- **Platform**: Takes 20% fee for hosting the service
- **Charity**: Receives configurable percentage (minimum 50% of remaining funds)
- **Host**: Quiz creator who can set optional host fee (max 5%)
- **Players**: Participants who pay entry fees and optional extras
- **Winners**: Top 1-3 players who receive prizes
- **Admin**: Contract administrator with management privileges

---

## Architecture

### Contract Structure

```
QuizRoomContract
├── Admin Management (initialization, roles, emergency controls)
├── Token Management (allowlist, approval, validation)
├── Room Creation (pool-based and asset-based)
├── Player Management (joining, validation, optimization)
├── Game Completion (winner selection, prize distribution)
├── Security Layer (reentrancy, overflow, validation)
└── Query Functions (read-only data access)
```

### Data Structures

#### Core Types
```rust
pub enum PrizeMode {
    PrizePoolSplit,  // Prizes from collected fees
    AssetBased,      // Pre-escrowed prize assets
}

pub enum Role {
    Admin,     // Full contract control
    Host,      // Can create rooms
    Player,    // Can join rooms
    Emergency, // Can pause contract
}
```

#### Key Structs
- **RoomConfig**: Complete room state and configuration
- **PlayerEntry**: Individual player data and payments
- **TokenInfo**: Approved token metadata
- **AdminConfig**: Administrative addresses and settings
- **EconomicConfig**: Financial parameters and limits

---

## Security Features

### 🛡️ Comprehensive Protection

1. **Integer Overflow/Underflow Protection**
   - All arithmetic uses checked operations
   - Safe math helper functions
   - Explicit error handling for edge cases

2. **Reentrancy Protection**
   - Storage-based reentrancy guards
   - Checks-effects-interactions pattern
   - State updates before external calls

3. **Token Transfer Security**
   - Balance verification before/after transfers
   - Transfer success validation
   - Proper error propagation

4. **Access Control**
   - Role-based permissions
   - Emergency pause functionality
   - Secure admin transfer process

5. **Input Validation**
   - Comprehensive address validation
   - Token contract verification
   - Range checking for all parameters

6. **State Consistency**
   - Atomic updates with rollback
   - State validation after changes
   - Consistent storage patterns

---

## Deployment Guide

### Prerequisites
- Rust and Soroban CLI installed
- Access to Stellar testnet/mainnet
- Funded accounts for deployment and testing

### Step 1: Build Contract
```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Optimize the WASM
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/quiz_room.wasm
```

### Step 2: Deploy to Network
```bash
# Deploy to testnet
CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/quiz_room.wasm \
  --source-account alice \
  --network testnet)

echo "Contract deployed at: $CONTRACT_ID"
```

### Step 3: Initialize Contract
```bash
# Set your addresses
ADMIN_ADDRESS="GBZXN7PIRZGNMHGA7MUUUF4GWJQ5FMHP22FZY2X7ZLGMWHS7CGPAB123"
PLATFORM_WALLET="GCBZQUCR63FVZY7PLWCIV5S4FOOK2KR2YRAHIFAY34MWLF5VYR7CJABV"
CHARITY_WALLET="GCZQUG5VNZGQ7D67GK7BHVPSLJUGZQEU7WZ5LJNF57HBUABQ234567"

# Initialize the contract
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account alice \
  --network testnet \
  -- \
  initialize \
  --admin $ADMIN_ADDRESS \
  --platform_wallet $PLATFORM_WALLET \
  --charity_wallet $CHARITY_WALLET
```

### Step 4: Setup Approved Tokens
```bash
# Add USDC testnet token
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account alice \
  --network testnet \
  -- \
  add_approved_token \
  --token_address "CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA" \
  --symbol "USDC" \
  --name "USD Coin"

# Add more tokens as needed...
```

### Step 5: Verify Deployment
```bash
# Check approved tokens
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account alice \
  --network testnet \
  -- \
  get_approved_tokens_list

# Check admin configuration
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account alice \
  --network testnet \
  -- \
  get_platform_wallet
```

---

## API Reference

### Admin Functions

#### `initialize(admin, platform_wallet, charity_wallet)`
**Description**: One-time contract initialization
**Access**: Anyone (but only works once)
**Parameters**:
- `admin: Address` - Admin account address
- `platform_wallet: Address` - Platform fee destination
- `charity_wallet: Address` - Charity fee destination

**Example**:
```bash
initialize \
  --admin GBZXN7PIRZGNMHGA7MUUUF4GWJQ5FMHP22FZY2X7ZLGMWHS7CGPAB123 \
  --platform_wallet GCBZQUCR63FVZY7PLWCIV5S4FOOK2KR2YRAHIFAY34MWLF5VYR7CJABV \
  --charity_wallet GCZQUG5VNZGQ7D67GK7BHVPSLJUGZQEU7WZ5LJNF57HBUABQ234567
```

#### `transfer_admin(new_admin)`
**Description**: Initiate admin transfer to new address
**Access**: Current admin only
**Parameters**:
- `new_admin: Address` - New admin address

#### `accept_admin()`
**Description**: Accept pending admin transfer
**Access**: Pending admin only

#### `emergency_pause()` / `emergency_unpause()`
**Description**: Emergency contract pause/unpause
**Access**: Emergency role only

### Token Management

#### `add_approved_token(token_address, symbol, name)`
**Description**: Add token to approved list
**Access**: Admin only
**Parameters**:
- `token_address: Address` - Token contract address
- `symbol: String` - Token symbol (e.g., "USDC")
- `name: String` - Token name (e.g., "USD Coin")

#### `remove_approved_token(token_address)`
**Description**: Remove token from approved list
**Access**: Admin only

#### `enable_disable_token(token_address, enabled)`
**Description**: Enable/disable approved token
**Access**: Admin only

#### `get_approved_tokens_list()`
**Description**: Get all enabled approved tokens
**Access**: Public
**Returns**: `Vec<TokenInfo>`

### Room Creation

#### `init_pool_room(...)`
**Description**: Create prize pool split room
**Access**: Anyone (with valid token)
**Parameters**:
- `room_id: u32` - Unique room identifier
- `host: Address` - Room host address
- `fee_token: Address` - Approved token for fees
- `entry_fee: i128` - Base entry fee amount
- `host_fee_bps: Option<u32>` - Host fee (0-500 basis points)
- `prize_pool_bps: u32` - Prize pool percentage (0-2500 basis points)
- `first_place_pct: u32` - First place percentage (1-100)
- `second_place_pct: Option<u32>` - Second place percentage
- `third_place_pct: Option<u32>` - Third place percentage

**Fee Distribution**:
- Platform: 20% (fixed)
- Charity: Remaining after host + prize pool (min 50%)
- Host: 0-5% (optional)
- Prize Pool: 0-25% split among winners

**Example**:
```bash
init_pool_room \
  --room_id 1 \
  --host GBZXN7PIRZGNMHGA7MUUUF4GWJQ5FMHP22FZY2X7ZLGMWHS7CGPAB123 \
  --fee_token CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA \
  --entry_fee 10000000 \
  --host_fee_bps 250 \
  --prize_pool_bps 2000 \
  --first_place_pct 60 \
  --second_place_pct 30 \
  --third_place_pct 10
```

#### `init_asset_room(...)`
**Description**: Create asset-based prize room
**Access**: Anyone (with valid tokens)
**Parameters**:
- `room_id: u32` - Unique room identifier
- `host: Address` - Room host address
- `fee_token: Address` - Approved token for entry fees
- `entry_fee: i128` - Base entry fee amount
- `host_fee_bps: Option<u32>` - Host fee (0-500 basis points)
- `prizes: Vec<PrizeAsset>` - 1-3 prize assets to escrow

**Note**: Host must approve prize assets before calling this function.

### Player Management

#### `join_room(room_id, player, screen_name, extras_amount)`
**Description**: Join a quiz room
**Access**: Anyone
**Parameters**:
- `room_id: u32` - Room to join
- `player: Address` - Player address
- `screen_name: String` - Display name (unique per room)
- `extras_amount: i128` - Additional payment beyond entry fee

**Example**:
```bash
join_room \
  --room_id 1 \
  --player GCZQUG5VNZGQ7D67GK7BHVPSLJUGZQEU7WZ5LJNF57HBUABQ234567 \
  --screen_name "PlayerOne" \
  --extras_amount 1000000
```

### Game Completion

#### `end_room(room_id, first_place, second_place, third_place)`
**Description**: End room and distribute prizes by address
**Access**: Room host only
**Parameters**:
- `room_id: u32` - Room to end
- `first_place: Option<Address>` - First place winner
- `second_place: Option<Address>` - Second place winner (optional)
- `third_place: Option<Address>` - Third place winner (optional)

#### `end_room_by_screen_names(room_id, first_place_name, ...)`
**Description**: End room and distribute prizes by screen name
**Access**: Room host only

### Query Functions

#### `get_room_config(room_id)`
**Description**: Get complete room configuration
**Returns**: `Option<RoomConfig>`

#### `get_room_players(room_id)`
**Description**: Get all players in room
**Returns**: `Vec<PlayerEntry>`

#### `get_room_financials(room_id)`
**Description**: Get room financial summary
**Returns**: `Option<(total_pool, entry_fees, extras_fees, expected_payouts, remainder)>`

#### `get_player_by_screen_name(room_id, screen_name)`
**Description**: Get player address by screen name
**Returns**: `Option<Address>`

---

## Usage Examples

### Complete Room Lifecycle

#### 1. Create Pool-Based Room
```bash
# Host creates room
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account host \
  --network testnet \
  -- \
  init_pool_room \
  --room_id 1 \
  --host GBHOST123456789012345678901234567890123456789012 \
  --fee_token CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA \
  --entry_fee 5000000 \
  --host_fee_bps 200 \
  --prize_pool_bps 2000 \
  --first_place_pct 50 \
  --second_place_pct 30 \
  --third_place_pct 20
```

#### 2. Players Join Room
```bash
# Player 1 joins
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account player1 \
  --network testnet \
  -- \
  join_room \
  --room_id 1 \
  --player GBPLAYER1234567890123456789012345678901234567890 \
  --screen_name "QuizMaster" \
  --extras_amount 1000000

# Player 2 joins
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account player2 \
  --network testnet \
  -- \
  join_room \
  --room_id 1 \
  --player GBPLAYER2345678901234567890123456789012345678901 \
  --screen_name "BrainBox" \
  --extras_amount 500000

# Player 3 joins
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account player3 \
  --network testnet \
  -- \
  join_room \
  --room_id 1 \
  --player GBPLAYER3456789012345678901234567890123456789012 \
  --screen_name "Smarty" \
  --extras_amount 0
```

#### 3. Check Room Status
```bash
# Get room financials
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account anyone \
  --network testnet \
  -- \
  get_room_financials \
  --room_id 1

# Get all players
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account anyone \
  --network testnet \
  -- \
  get_room_players \
  --room_id 1
```

#### 4. End Room and Distribute Prizes
```bash
# Host ends room with winners
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account host \
  --network testnet \
  -- \
  end_room_by_screen_names \
  --room_id 1 \
  --first_place_name "BrainBox" \
  --second_place_name "QuizMaster" \
  --third_place_name "Smarty"
```

### Asset-Based Room Example

#### 1. Host Prepares Prize Assets
```bash
# Host approves prize tokens for contract
soroban contract invoke \
  --id PRIZE_TOKEN_CONTRACT \
  --source-account host \
  --network testnet \
  -- \
  approve \
  --from GBHOST123456789012345678901234567890123456789012 \
  --spender $CONTRACT_ID \
  --amount 1000000000
```

#### 2. Create Asset Room
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account host \
  --network testnet \
  -- \
  init_asset_room \
  --room_id 2 \
  --host GBHOST123456789012345678901234567890123456789012 \
  --fee_token CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA \
  --entry_fee 2000000 \
  --host_fee_bps 300 \
  --prizes '[
    {
      "contract_id": "CPRIZE1234567890123456789012345678901234567890",
      "amount": 100000000
    },
    {
      "contract_id": "CPRIZE2345678901234567890123456789012345678901", 
      "amount": 50000000
    },
    {
      "contract_id": "CPRIZE3456789012345678901234567890123456789012",
      "amount": 25000000
    }
  ]'
```

---

## Error Handling

### Error Types and Meanings

#### Critical Security Errors
- `ArithmeticOverflow` (26): Integer overflow detected
- `ArithmeticUnderflow` (27): Integer underflow detected  
- `ReentrancyDetected` (31): Reentrancy attack blocked
- `TransferVerificationFailed` (30): Token transfer validation failed
- `EmergencyPause` (40): Contract is paused

#### Validation Errors
- `InvalidAddress` (32): Invalid or malformed address
- `InvalidToken` (33): Invalid token contract
- `TokenNotApproved` (43): Token not in allowlist
- `InvalidScreenName` (25): Screen name validation failed
- `AmountTooLarge` (34): Amount exceeds safe limits

#### Business Logic Errors
- `RoomNotFound` (12): Room ID doesn't exist
- `RoomAlreadyExists` (11): Room ID already used
- `PlayerAlreadyJoined` (16): Player already in room
- `ScreenNameTaken` (24): Screen name already used
- `InsufficientPlayers` (21): Not enough players to end room
- `Unauthorized` (18): Insufficient permissions

#### Configuration Errors
- `NotInitialized` (37): Contract not initialized
- `AlreadyInitialized` (38): Contract already initialized
- `InvalidHostFee` (1): Host fee exceeds maximum
- `CharityBelowMinimum` (4): Charity percentage too low

### Error Handling Best Practices

1. **Always Check Return Values**
```rust
match contract.join_room(room_id, player, name, extras) {
    Ok(_) => println!("Successfully joined room"),
    Err(QuizError::PlayerAlreadyJoined) => println!("Already in this room"),
    Err(QuizError::ScreenNameTaken) => println!("Screen name unavailable"),
    Err(e) => println!("Error: {:?}", e),
}
```

2. **Validate Inputs Before Contract Calls**
```typescript
// Frontend validation
function validateJoinRoom(screenName: string, extras: number) {
    if (screenName.length === 0 || screenName.length > 20) {
        throw new Error("Screen name must be 1-20 characters");
    }
    if (extras < 0) {
        throw new Error("Extras amount cannot be negative");
    }
    if (!/^[a-zA-Z0-9_\- ]+$/.test(screenName)) {
        throw new Error("Screen name contains invalid characters");
    }
}
```

3. **Handle Network Errors**
```typescript
async function joinRoom(roomId: number, player: string, screenName: string, extras: number) {
    try {
        validateJoinRoom(screenName, extras);
        const result = await contract.join_room({
            room_id: roomId,
            player,
            screen_name: screenName,
            extras_amount: extras
        });
        return { success: true, result };
    } catch (error) {
        if (error.message.includes("TokenNotApproved")) {
            return { success: false, error: "Token not supported" };
        }
        if (error.message.includes("InsufficientBalance")) {
            return { success: false, error: "Insufficient token balance" };
        }
        return { success: false, error: error.message };
    }
}
```

---

## Frontend Integration

### TypeScript Integration

#### Contract Interface Definition
```typescript
interface QuizRoomContract {
    // Admin functions
    initialize(admin: string, platformWallet: string, charityWallet: string): Promise<void>;
    addApprovedToken(tokenAddress: string, symbol: string, name: string): Promise<void>;
    
    // Room management
    initPoolRoom(params: PoolRoomParams): Promise<void>;
    initAssetRoom(params: AssetRoomParams): Promise<void>;
    joinRoom(params: JoinRoomParams): Promise<void>;
    endRoom(params: EndRoomParams): Promise<void>;
    
    // Queries
    getApprovedTokensList(): Promise<TokenInfo[]>;
    getRoomConfig(roomId: number): Promise<RoomConfig | null>;
    getRoomPlayers(roomId: number): Promise<PlayerEntry[]>;
    getRoomFinancials(roomId: number): Promise<FinancialSummary | null>;
}

interface TokenInfo {
    contract_id: string;
    symbol: string;
    name: string;
    decimals: number;
    enabled: boolean;
}

interface RoomConfig {
    room_id: string;
    host: string;
    fee_token: string;
    entry_fee: string;
    host_fee_bps: number;
    prize_mode: "PrizePoolSplit" | "AssetBased";
    ended: boolean;
    player_count: number;
    total_pool: string;
    winners: string[];
}
```

#### React Hook Example
```typescript
import { useState, useEffect } from 'react';

export function useQuizRoom(contractId: string) {
    const [approvedTokens, setApprovedTokens] = useState<TokenInfo[]>([]);
    const [loading, setLoading] = useState(true);
    
    useEffect(() => {
        loadApprovedTokens();
    }, [contractId]);
    
    const loadApprovedTokens = async () => {
        try {
            const tokens = await contract.getApprovedTokensList();
            setApprovedTokens(tokens);
        } catch (error) {
            console.error("Failed to load approved tokens:", error);
        } finally {
            setLoading(false);
        }
    };
    
    const createRoom = async (params: PoolRoomParams) => {
        try {
            await contract.initPoolRoom(params);
            return { success: true };
        } catch (error) {
            return { success: false, error: error.message };
        }
    };
    
    const joinRoom = async (roomId: number, screenName: string, extras: number) => {
        try {
            await contract.joinRoom({
                room_id: roomId,
                player: wallet.publicKey,
                screen_name: screenName,
                extras_amount: extras
            });
            return { success: true };
        } catch (error) {
            return { success: false, error: error.message };
        }
    };
    
    return {
        approvedTokens,
        loading,
        createRoom,
        joinRoom,
        loadApprovedTokens
    };
}
```

#### Room Creation Form Component
```typescript
import React, { useState } from 'react';

interface CreateRoomFormProps {
    approvedTokens: TokenInfo[];
    onCreateRoom: (params: PoolRoomParams) => Promise<{success: boolean, error?: string}>;
}

export function CreateRoomForm({ approvedTokens, onCreateRoom }: CreateRoomFormProps) {
    const [formData, setFormData] = useState({
        roomId: '',
        feeToken: '',
        entryFee: '',
        hostFee: '0',
        prizePool: '2000',
        firstPlace: '60',
        secondPlace: '30',
        thirdPlace: '10'
    });
    
    const [creating, setCreating] = useState(false);
    const [error, setError] = useState('');
    
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setCreating(true);
        setError('');
        
        try {
            const params = {
                room_id: parseInt(formData.roomId),
                host: wallet.publicKey,
                fee_token: formData.feeToken,
                entry_fee: (parseFloat(formData.entryFee) * 10000000).toString(), // Convert to stroops
                host_fee_bps: parseInt(formData.hostFee),
                prize_pool_bps: parseInt(formData.prizePool),
                first_place_pct: parseInt(formData.firstPlace),
                second_place_pct: parseInt(formData.secondPlace) || null,
                third_place_pct: parseInt(formData.thirdPlace) || null
            };
            
            const result = await onCreateRoom(params);
            if (!result.success) {
                setError(result.error || 'Failed to create room');
            }
        } catch (err) {
            setError('Unexpected error occurred');
        } finally {
            setCreating(false);
        }
    };
    
    return (
        <form onSubmit={handleSubmit} className="space-y-4">
            <div>
                <label>Room ID</label>
                <input
                    type="number"
                    value={formData.roomId}
                    onChange={(e) => setFormData({...formData, roomId: e.target.value})}
                    required
                />
            </div>
            
            <div>
                <label>Fee Token</label>
                <select
                    value={formData.feeToken}
                    onChange={(e) => setFormData({...formData, feeToken: e.target.value})}
                    required
                >
                    <option value="">Select token...</option>
                    {approvedTokens.map(token => (
                        <option key={token.contract_id} value={token.contract_id}>
                            {token.symbol} - {token.name}
                        </option>
                    ))}
                </select>
            </div>
            
            <div>
                <label>Entry Fee</label>
                <input
                    type="number"
                    step="0.0000001"
                    value={formData.entryFee}
                    onChange={(e) => setFormData({...formData, entryFee: e.target.value})}
                    required
                />
            </div>
            
            {/* Additional form fields... */}
            
            {error && (
                <div className="text-red-600 bg-red-100 p-2 rounded">
                    {error}
                </div>
            )}
            
            <button
                type="submit"
                disabled={creating}
                className="bg-blue-600 text-white px-4 py-2 rounded disabled:opacity-50"
            >
                {creating ? 'Creating...' : 'Create Room'}
            </button>
        </form>
    );
}
```

### Real-time Updates with Events

```typescript
export function useRoomEvents(contractId: string, roomId: number) {
    const [players, setPlayers] = useState<PlayerEntry[]>([]);
    const [roomStatus, setRoomStatus] = useState<'active' | 'ended'>('active');
    
    useEffect(() => {
        // Subscribe to contract events
        const eventSubscription = sorobanClient.events({
            contractIds: [contractId],
            topics: [['player_joined'], ['game_ended']],
            startLedger: 'latest'
        });
        
        eventSubscription.on('data', (event) => {
            if (event.topic[0] === 'player_joined') {
                // Refresh player list
                loadPlayers();
            } else if (event.topic[0] === 'game_ended') {
                setRoomStatus('ended');
                loadPlayers(); // Refresh to show final state
            }
        });
        
        return () => eventSubscription.close();
    }, [contractId, roomId]);
    
    const loadPlayers = async () => {
        try {
            const playerList = await contract.getRoomPlayers(roomId);
            setPlayers(playerList);
        } catch (error) {
            console.error('Failed to load players:', error);
        }
    };
    
    return { players, roomStatus, loadPlayers };
}
```

---

## Testing Guide

### Unit Testing Framework

#### Test Setup
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        Address, Env
    };
    
    fn create_test_contract(e: &Env) -> Address {
        e.register_contract(None, QuizRoomContract)
    }
    
    fn initialize_test_contract(
        e: &Env,
        contract_id: &Address,
        admin: &Address,
    ) -> Result<(), QuizError> {
        let platform = Address::generate(e);
        let charity = Address::generate(e);
        
        QuizRoomContract::initialize(e, admin.clone(), platform, charity)
    }
    
    #[test]
    fn test_initialization() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let contract_id = create_test_contract(&e);
        
        // Test successful initialization
        assert!(initialize_test_contract(&e, &contract_id, &admin).is_ok());
        
        // Test double initialization fails
        assert_eq!(
            initialize_test_contract(&e, &contract_id, &admin),
            Err(QuizError::AlreadyInitialized)
        );
    }
}
```

#### Security Tests
```rust
#[cfg(test)]
mod security_tests {
    use super::*;
    
    #[test]
    fn test_integer_overflow_protection() {
        let e = Env::default();
        
        // Test safe_add with overflow
        let result = QuizRoomContract::safe_add(i128::MAX, 1);
        assert_eq!(result, Err(QuizError::ArithmeticOverflow));
        
        // Test safe_mul with overflow
        let result = QuizRoomContract::safe_mul(i128::MAX, 2);
        assert_eq!(result, Err(QuizError::ArithmeticOverflow));
        
        // Test safe operations within bounds
        let result = QuizRoomContract::safe_add(100, 200);
        assert_eq!(result, Ok(300));
    }
    
    #[test]
    fn test_reentrancy_protection() {
        let e = Env::default();
        let contract_id = create_test_contract(&e);
        
        // Test reentrancy guard works
        QuizRoomContract::set_reentrancy_guard(&e);
        let result = QuizRoomContract::check_reentrancy(&e);
        assert_eq!(result, Err(QuizError::ReentrancyDetected));
        
        // Test guard clears properly
        QuizRoomContract::clear_reentrancy_guard(&e);
        let result = QuizRoomContract::check_reentrancy(&e);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_token_validation() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let contract_id = create_test_contract(&e);
        
        initialize_test_contract(&e, &contract_id, &admin).unwrap();
        
        // Test invalid token rejection
        let invalid_token = Address::generate(&e);
        let result = QuizRoomContract::validate_approved_token(&e, &invalid_token);
        assert_eq!(result, Err(QuizError::TokenNotApproved));
    }
    
    #[test]
    fn test_input_validation() {
        let e = Env::default();
        
        // Test screen name validation
        let valid_name = String::from_str(&e, "ValidName123");
        assert!(QuizRoomContract::validate_screen_name(&valid_name).is_ok());
        
        let invalid_name = String::from_str(&e, "Invalid@Name#");
        assert_eq!(
            QuizRoomContract::validate_screen_name(&invalid_name),
            Err(QuizError::InvalidScreenName)
        );
        
        let too_long_name = String::from_str(&e, "ThisNameIsWayTooLongForValidation");
        assert_eq!(
            QuizRoomContract::validate_screen_name(&too_long_name),
            Err(QuizError::InvalidScreenName)
        );
        
        // Test amount validation
        assert!(QuizRoomContract::validate_amount(1000, 0).is_ok());
        assert_eq!(
            QuizRoomContract::validate_amount(-1, 0),
            Err(QuizError::InsufficientAmount)
        );
        assert_eq!(
            QuizRoomContract::validate_amount(i128::MAX, 0),
            Err(QuizError::AmountTooLarge)
        );
    }
}
```

#### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_complete_room_lifecycle() {
        let e = Env::default();
        e.mock_all_auths();
        
        let admin = Address::generate(&e);
        let host = Address::generate(&e);
        let player1 = Address::generate(&e);
        let player2 = Address::generate(&e);
        let player3 = Address::generate(&e);
        
        let contract_id = create_test_contract(&e);
        
        // Initialize contract
        initialize_test_contract(&e, &contract_id, &admin).unwrap();
        
        // Add approved token
        let token_id = e.register_stellar_asset_contract(admin.clone());
        QuizRoomContract::add_approved_token(
            &e,
            token_id.clone(),
            String::from_str(&e, "TEST"),
            String::from_str(&e, "Test Token")
        ).unwrap();
        
        // Create room
        QuizRoomContract::init_pool_room(
            &e,
            1,
            host.clone(),
            token_id.clone(),
            1000000, // 0.1 tokens
            Some(250), // 2.5% host fee
            2000, // 20% prize pool
            60, // 60% first place
            Some(30), // 30% second place
            Some(10), // 10% third place
        ).unwrap();
        
        // Players join
        QuizRoomContract::join_room(
            &e,
            1,
            player1.clone(),
            String::from_str(&e, "Player1"),
            500000, // 0.05 tokens extras
        ).unwrap();
        
        QuizRoomContract::join_room(
            &e,
            1,
            player2.clone(),
            String::from_str(&e, "Player2"),
            0,
        ).unwrap();
        
        QuizRoomContract::join_room(
            &e,
            1,
            player3.clone(),
            String::from_str(&e, "Player3"),
            250000, // 0.025 tokens extras
        ).unwrap();
        
        // Check room state
        let config = QuizRoomContract::get_room_config(&e, 1).unwrap();
        assert_eq!(config.player_count, 3);
        assert_eq!(config.total_pool, 3750000); // 3 * 1000000 + 750000 extras
        
        // End room
        QuizRoomContract::end_room(
            &e,
            1,
            Some(player2.clone()), // Winner
            Some(player1.clone()), // Second
            Some(player3.clone()), // Third
        ).unwrap();
        
        // Verify room ended
        let final_config = QuizRoomContract::get_room_config(&e, 1).unwrap();
        assert!(final_config.ended);
        assert_eq!(final_config.winners.len(), 3);
    }
    
    #[test]
    fn test_financial_calculations() {
        let e = Env::default();
        e.mock_all_auths();
        
        let admin = Address::generate(&e);
        let host = Address::generate(&e);
        let contract_id = create_test_contract(&e);
        
        initialize_test_contract(&e, &contract_id, &admin).unwrap();
        
        let token_id = e.register_stellar_asset_contract(admin.clone());
        QuizRoomContract::add_approved_token(
            &e,
            token_id.clone(),
            String::from_str(&e, "TEST"),
            String::from_str(&e, "Test Token")
        ).unwrap();
        
        // Create room with specific fees
        QuizRoomContract::init_pool_room(
            &e,
            1,
            host.clone(),
            token_id.clone(),
            10000000, // 1 token entry fee
            Some(500), // 5% host fee
            2500, // 25% prize pool
            100, // 100% to first place
            None,
            None,
        ).unwrap();
        
        // Add players
        let player1 = Address::generate(&e);
        let player2 = Address::generate(&e);
        
        QuizRoomContract::join_room(
            &e,
            1,
            player1.clone(),
            String::from_str(&e, "Player1"),
            0,
        ).unwrap();
        
        QuizRoomContract::join_room(
            &e,
            1,
            player2.clone(),
            String::from_str(&e, "Player2"),
            5000000, // 0.5 tokens extras
        ).unwrap();
        
        // Check financials
        let financials = QuizRoomContract::get_room_financials(&e, 1).unwrap();
        let (total_pool, entry_fees, extras_fees, expected_payouts, remainder) = financials;
        
        assert_eq!(total_pool, 25000000); // 2 * 10M + 5M extras
        assert_eq!(entry_fees, 20000000); // 2 * 10M
        assert_eq!(extras_fees, 5000000); // 5M extras
        
        // Expected distribution:
        // Platform: 20% of 25M = 5M
        // Host: 5% of 25M = 1.25M
        // Prize: 25% of 25M = 6.25M
        // Charity: 50% of 25M = 12.5M
        // Total: 25M
        
        let expected_total = 5000000 + 1250000 + 6250000 + 12500000;
        assert_eq!(expected_payouts, expected_total);
        assert_eq!(remainder, 0);
    }
}
```

#### Property-Based Testing
```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn financial_calculations_never_exceed_total_pool(
            total_pool in 1i128..1_000_000_000i128,
            host_fee_bps in 0u32..500u32,
            prize_pool_bps in 0u32..2500u32,
        ) {
            let platform_bps = 2000u32; // Fixed 20%
            let charity_bps = 10000u32 - platform_bps - host_fee_bps - prize_pool_bps;
            
            // Skip invalid combinations
            prop_assume!(charity_bps >= 5000); // Min 50% charity
            
            let platform_amount = (total_pool * platform_bps as i128) / 10000;
            let charity_amount = (total_pool * charity_bps as i128) / 10000;
            let host_amount = (total_pool * host_fee_bps as i128) / 10000;
            let prize_amount = (total_pool * prize_pool_bps as i128) / 10000;
            
            let total_distributed = platform_amount + charity_amount + host_amount + prize_amount;
            
            // Should never exceed total pool
            prop_assert!(total_distributed <= total_pool);
            
            // Should be very close to total pool (within rounding)
            prop_assert!(total_pool - total_distributed < 4); // Max 3 units rounding error
        }
        
        #[test]
        fn safe_math_operations_consistent(
            a in 0i128..1_000_000i128,
            b in 0i128..1_000_000i128,
        ) {
            // Test that safe operations match regular operations for small numbers
            prop_assert_eq!(
                QuizRoomContract::safe_add(a, b).unwrap(),
                a + b
            );
            
            prop_assert_eq!(
                QuizRoomContract::safe_mul(a, b).unwrap(),
                a * b
            );
            
            if b != 0 {
                prop_assert_eq!(
                    QuizRoomContract::safe_div(a, b).unwrap(),
                    a / b
                );
            }
        }
    }
}
```

### Performance Testing
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_large_room_performance() {
        let e = Env::default();
        e.mock_all_auths();
        
        let admin = Address::generate(&e);
        let host = Address::generate(&e);
        let contract_id = create_test_contract(&e);
        
        initialize_test_contract(&e, &contract_id, &admin).unwrap();
        
        let token_id = e.register_stellar_asset_contract(admin.clone());
        QuizRoomContract::add_approved_token(
            &e,
            token_id.clone(),
            String::from_str(&e, "TEST"),
            String::from_str(&e, "Test Token")
        ).unwrap();
        
        // Create room
        QuizRoomContract::init_pool_room(
            &e,
            1,
            host.clone(),
            token_id.clone(),
            1000000,
            None,
            2000,
            100,
            None,
            None,
        ).unwrap();
        
        // Add many players (test O(1) performance)
        let start_time = std::time::Instant::now();
        
        for i in 0..100 {
            let player = Address::generate(&e);
            let screen_name = String::from_str(&e, &format!("Player{}", i));
            
            QuizRoomContract::join_room(
                &e,
                1,
                player,
                screen_name,
                0,
            ).unwrap();
        }
        
        let join_duration = start_time.elapsed();
        
        // Test player lookup performance (should be O(1))
        let lookup_start = std::time::Instant::now();
        
        for i in 0..100 {
            let screen_name = String::from_str(&e, &format!("Player{}", i));
            let _player = QuizRoomContract::get_player_by_screen_name(&e, 1, screen_name);
        }
        
        let lookup_duration = lookup_start.elapsed();
        
        println!("Join 100 players: {:?}", join_duration);
        println!("Lookup 100 players: {:?}", lookup_duration);
        
        // Performance assertions (adjust based on your requirements)
        assert!(join_duration.as_millis() < 1000); // Should complete in under 1 second
        assert!(lookup_duration.as_millis() < 100); // Lookups should be very fast
    }
}
```

---

## Maintenance & Operations

### Admin Operations

#### Regular Maintenance Tasks
```bash
#!/bin/bash
# maintenance.sh - Regular admin tasks

CONTRACT_ID="your_contract_id"
ADMIN_ACCOUNT="admin_account"
NETWORK="testnet" # or "mainnet"

# 1. Check contract health
echo "Checking contract status..."
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_ACCOUNT \
  --network $NETWORK \
  -- \
  is_emergency_paused

# 2. Review approved tokens
echo "Current approved tokens:"
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_ACCOUNT \
  --network $NETWORK \
  -- \
  get_approved_tokens_list

# 3. Check platform wallet balance
echo "Platform wallet info:"
PLATFORM_WALLET=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_ACCOUNT \
  --network $NETWORK \
  -- \
  get_platform_wallet)

echo "Platform wallet: $PLATFORM_WALLET"

# 4. Monitor for unusual activity (implement custom logic)
echo "Monitoring recent transactions..."
# Add monitoring logic here
```

#### Token Management Operations
```bash
#!/bin/bash
# token_management.sh

# Add new token
add_token() {
    local token_address=$1
    local symbol=$2
    local name=$3
    
    echo "Adding token: $symbol ($name)"
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source-account $ADMIN_ACCOUNT \
      --network $NETWORK \
      -- \
      add_approved_token \
      --token_address $token_address \
      --symbol $symbol \
      --name $name
}

# Disable problematic token
disable_token() {
    local token_address=$1
    
    echo "Disabling token: $token_address"
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source-account $ADMIN_ACCOUNT \
      --network $NETWORK \
      -- \
      enable_disable_token \
      --token_address $token_address \
      --enabled false
}

# Remove token completely
remove_token() {
    local token_address=$1
    
    echo "Removing token: $token_address"
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source-account $ADMIN_ACCOUNT \
      --network $NETWORK \
      -- \
      remove_approved_token \
      --token_address $token_address
}

# Usage examples:
# add_token "CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA" "USDC" "USD Coin"
# disable_token "CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA"
# remove_token "CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA"
```

### Monitoring & Alerting

#### Event Monitoring
```typescript
// monitoring.ts - Event monitoring system
import { SorobanRpc } from 'soroban-client';

interface ContractEvent {
    type: string;
    contractId: string;
    topic: string[];
    value: any;
    ledger: number;
    txHash: string;
}

class QuizRoomMonitor {
    private server: SorobanRpc.Server;
    private contractId: string;
    private lastProcessedLedger: number;
    
    constructor(serverUrl: string, contractId: string) {
        this.server = new SorobanRpc.Server(serverUrl);
        this.contractId = contractId;
        this.lastProcessedLedger = 0;
    }
    
    async startMonitoring() {
        console.log('Starting contract monitoring...');
        
        while (true) {
            try {
                await this.processEvents();
                await this.sleep(5000); // Check every 5 seconds
            } catch (error) {
                console.error('Monitoring error:', error);
                await this.sleep(10000); // Wait longer on error
            }
        }
    }
    
    private async processEvents() {
        const events = await this.server.getEvents({
            startLedger: this.lastProcessedLedger + 1,
            filters: [{
                type: 'contract',
                contractIds: [this.contractId]
            }]
        });
        
        for (const event of events.events) {
            await this.handleEvent({
                type: event.type,
                contractId: event.contractId,
                topic: event.topic,
                value: event.value,
                ledger: event.ledger,
                txHash: event.txHash
            });
            
            this.lastProcessedLedger = Math.max(this.lastProcessedLedger, event.ledger);
        }
    }
    
    private async handleEvent(event: ContractEvent) {
        const eventType = event.topic[0];
        
        switch (eventType) {
            case 'pool_room_created':
                await this.onRoomCreated(event);
                break;
            case 'player_joined':
                await this.onPlayerJoined(event);
                break;
            case 'game_ended':
                await this.onGameEnded(event);
                break;
            case 'emergency_pause':
                await this.onEmergencyPause(event);
                break;
            case 'security_alert':
                await this.onSecurityAlert(event);
                break;
            default:
                console.log('Unknown event type:', eventType);
        }
    }
    
    private async onRoomCreated(event: ContractEvent) {
        console.log('New room created:', event.value);
        
        // Alert if unusual entry fee
        const entryFee = parseInt(event.value.entry_fee);
        if (entryFee > 100000000) { // > 10 tokens
            await this.sendAlert('High entry fee room created', event);
        }
    }
    
    private async onPlayerJoined(event: ContractEvent) {
        console.log('Player joined:', event.value);
        
        // Track player activity
        await this.recordPlayerActivity(event.value.player, event.value.room_id);
    }
    
    private async onGameEnded(event: ContractEvent) {
        console.log('Game ended:', event.value);
        
        // Record game completion stats
        await this.recordGameStats(event.value);
    }
    
    private async onEmergencyPause(event: ContractEvent) {
        console.log('EMERGENCY PAUSE ACTIVATED:', event.value);
        
        // Send immediate alert
        await this.sendCriticalAlert('Contract emergency pause activated', event);
    }
    
    private async onSecurityAlert(event: ContractEvent) {
        console.log('SECURITY ALERT:', event.value);
        
        // Send security alert
        await this.sendSecurityAlert('Security event detected', event);
    }
    
    private async sendAlert(message: string, event: ContractEvent) {
        console.log(`ALERT: ${message}`, event);
        // Implement your alerting mechanism (email, Slack, etc.)
    }
    
    private async sendCriticalAlert(message: string, event: ContractEvent) {
        console.log(`CRITICAL ALERT: ${message}`, event);
        // Implement critical alerting (SMS, phone call, etc.)
    }
    
    private async sendSecurityAlert(message: string, event: ContractEvent) {
        console.log(`SECURITY ALERT: ${message}`, event);
        // Implement security alerting
    }
    
    private async recordPlayerActivity(player: string, roomId: number) {
        // Record to analytics database
    }
    
    private async recordGameStats(gameData: any) {
        // Record game completion metrics
    }
    
    private sleep(ms: number): Promise<void> {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
}

// Usage
const monitor = new QuizRoomMonitor(
    'https://soroban-testnet.stellar.org',
    'CONTRACT_ID_HERE'
);

monitor.startMonitoring();
```

#### Health Check Dashboard
```typescript
// health_check.ts
interface HealthStatus {
    contractAccessible: boolean;
    emergencyPaused: boolean;
    approvedTokenCount: number;
    activeRooms: number;
    lastActivity: Date;
    platformWalletBalance: string;
    charityWalletBalance: string;
}

class HealthChecker {
    async checkContractHealth(contractId: string): Promise<HealthStatus> {
        try {
            // Check if contract is accessible
            const isAccessible = await this.testContractAccess(contractId);
            
            // Check emergency pause status
            const isPaused = await contract.is_emergency_paused();
            
            // Get approved token count
            const tokens = await contract.get_approved_tokens_list();
            
            // Check wallet balances
            const platformWallet = await contract.get_platform_wallet();
            const charityWallet = await contract.get_charity_wallet();
            
            const platformBalance = await this.getTokenBalance(platformWallet, 'USDC');
            const charityBalance = await this.getTokenBalance(charityWallet, 'USDC');
            
            return {
                contractAccessible: isAccessible,
                emergencyPaused: isPaused,
                approvedTokenCount: tokens.length,
                activeRooms: await this.countActiveRooms(contractId),
                lastActivity: new Date(),
                platformWalletBalance: platformBalance,
                charityWalletBalance: charityBalance
            };
        } catch (error) {
            throw new Error(`Health check failed: ${error.message}`);
        }
    }
    
    private async testContractAccess(contractId: string): Promise<boolean> {
        try {
            await contract.get_approved_tokens_list();
            return true;
        } catch {
            return false;
        }
    }
    
    private async countActiveRooms(contractId: string): Promise<number> {
        // Implement room counting logic
        // This might involve scanning recent events or maintaining a separate index
        return 0;
    }
    
    private async getTokenBalance(wallet: string, tokenSymbol: string): Promise<string> {
        // Implement token balance checking
        return "0";
    }
}
```

### Upgrade Procedures

#### Contract Upgrade Strategy
Since Soroban contracts are immutable, upgrades require a new deployment and migration strategy:

```bash
#!/bin/bash
# upgrade.sh - Contract upgrade procedure

OLD_CONTRACT_ID="old_contract_id"
ADMIN_ACCOUNT="admin_account"
NETWORK="testnet"

echo "Starting contract upgrade procedure..."

# 1. Deploy new contract version
echo "Deploying new contract..."
NEW_CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/quiz_room_v2.wasm \
  --source-account $ADMIN_ACCOUNT \
  --network $NETWORK)

echo "New contract deployed: $NEW_CONTRACT_ID"

# 2. Pause old contract
echo "Pausing old contract..."
soroban contract invoke \
  --id $OLD_CONTRACT_ID \
  --source-account $ADMIN_ACCOUNT \
  --network $NETWORK \
  -- \
  emergency_pause

# 3. Initialize new contract
echo "Initializing new contract..."
PLATFORM_WALLET=$(soroban contract invoke \
  --id $OLD_CONTRACT_ID \
  --source-account $ADMIN_ACCOUNT \
  --network $NETWORK \
  -- \
  get_platform_wallet)

CHARITY_WALLET=$(soroban contract invoke \
  --id $OLD_CONTRACT_ID \
  --source-account $ADMIN_ACCOUNT \
  --network $NETWORK \
  -- \
  get_charity_wallet)

soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $ADMIN_ACCOUNT \
  --network $NETWORK \
  -- \
  initialize \
  --admin $ADMIN_ACCOUNT \
  --platform_wallet $PLATFORM_WALLET \
  --charity_wallet $CHARITY_WALLET

# 4. Migrate approved tokens
echo "Migrating approved tokens..."
TOKENS=$(soroban contract invoke \
  --id $OLD_CONTRACT_ID \
  --source-account $ADMIN_ACCOUNT \
  --network $NETWORK \
  -- \
  get_approved_tokens_list)

# Parse tokens and add to new contract
# (Implementation depends on JSON parsing tools available)

# 5. Update frontend configuration
echo "Update frontend to use new contract: $NEW_CONTRACT_ID"

# 6. Monitor for issues
echo "Monitor new contract for 24-48 hours before considering migration complete"
```

### Disaster Recovery

#### Emergency Procedures
```bash
#!/bin/bash
# emergency.sh - Emergency response procedures

CONTRACT_ID="your_contract_id"
ADMIN_ACCOUNT="admin_account"
NETWORK="mainnet"

# Emergency pause (immediate)
emergency_pause() {
    echo "EMERGENCY: Pausing contract immediately"
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source-account $ADMIN_ACCOUNT \
      --network $NETWORK \
      -- \
      emergency_pause
    
    echo "Contract paused. Investigate issue immediately."
}

# Disable problematic token
disable_token_emergency() {
    local token_address=$1
    echo "EMERGENCY: Disabling token $token_address"
    
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source-account $ADMIN_ACCOUNT \
      --network $NETWORK \
      -- \
      enable_disable_token \
      --token_address $token_address \
      --enabled false
}

# Update wallet addresses if compromised
update_compromised_wallets() {
    local new_platform_wallet=$1
    local new_charity_wallet=$2
    
    echo "EMERGENCY: Updating wallet addresses"
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source-account $ADMIN_ACCOUNT \
      --network $NETWORK \
      -- \
      update_wallets \
      --platform_wallet $new_platform_wallet \
      --charity_wallet $new_charity_wallet
}

# Usage examples:
# emergency_pause
# disable_token_emergency "CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA"
# update_compromised_wallets "NEW_PLATFORM_WALLET" "NEW_CHARITY_WALLET"
```

---

## Conclusion

This documentation provides a comprehensive guide to deploying, operating, and maintaining the Soroban Quiz Room Contract. The contract implements enterprise-grade security features while maintaining usability and flexibility.

### Key Security Achievements
- ✅ **Zero integer overflow/underflow risk**
- ✅ **Complete reentrancy protection**
- ✅ **Robust token transfer validation**
- ✅ **Comprehensive input validation**
- ✅ **Emergency controls and admin management**
- ✅ **Gas-optimized operations**
- ✅ **State consistency guarantees**

### Production Readiness Checklist
- [ ] Deploy to testnet and run full test suite
- [ ] Configure monitoring and alerting systems
- [ ] Set up admin access controls and procedures
- [ ] Configure approved tokens for your use case
- [ ] Run security audit with external firm
- [ ] Implement frontend integration
- [ ] Create disaster recovery procedures
- [ ] Train operational team on emergency procedures
- [ ] Deploy to mainnet with limited exposure initially
- [ ] Gradually scale up based on monitoring results

For additional support or questions about implementation, refer to the Soroban documentation at [https://soroban.stellar.org](https://soroban.stellar.org) or the Stellar developer Discord community.