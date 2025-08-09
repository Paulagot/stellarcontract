# Quiz Room Smart Contract

A Stellar smart contract system for hosting quiz competitions with entry fees, prize distribution, and player management.

## Features

- 🎯 **Pool-based and Asset-based prize modes**
- 💰 **Configurable fee distribution** (host, platform, charity)
- 👥 **Player management** with screen names and entry tracking
- 🏆 **Flexible prize distribution** (1st, 2nd, 3rd place)
- 🔒 **Built-in safety checks** and validation
- 🌐 **Frontend integration** with auto-generated TypeScript clients

## Architecture

This project uses [Scaffold Stellar](https://github.com/AhaLabs/scaffold-stellar) for rapid smart contract development with:

- ⚡️ Vite + React + TypeScript frontend
- 🔗 Auto-generated contract clients
- 🧩 Ready-to-use components for contract interaction
- 🛠 Hot reload for contract changes
- 🧪 Testnet deployment support

## Project Structure

```
quiz-room-contract/
├── contracts/
│   ├── quiz/                    # Main quiz room contract
│   └── fungible-token-interface/ # Example token contract
├── packages/                    # Auto-generated TypeScript clients
├── src/                         # Frontend React application
│   ├── components/              # React components
│   ├── contracts/               # Contract interaction helpers
│   ├── debug/                   # Debugging contract explorer
│   ├── hooks/                   # Custom React hooks
│   └── pages/                   # App pages
├── target/                      # Build artifacts and WASM files
├── environments.toml            # Environment configurations
└── .env                         # Local environment variables
```

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) with wasm32 target
- [Node.js](https://nodejs.org/) (v22+)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)
- [Scaffold Stellar CLI Plugin](https://github.com/AhaLabs/scaffold-stellar)

### Local Development

1. **Clone and install dependencies:**
   ```bash
   git clone <your-repo>
   cd quiz-room-contract
   npm install
   ```

2. **Start local development:**
   ```bash
   npm run dev
   ```

### Testnet Deployment

> **Note**: Due to current Scaffold Stellar limitations, we recommend direct deployment for testnet.

1. **Set up testnet account:**
   ```bash
   stellar keys generate testnet-dev --network testnet
   stellar keys fund testnet-dev --network testnet
   ```

2. **Build and deploy contracts:**
   ```bash
   stellar contract build
   
   # Deploy token contract
   stellar contract deploy \
     --wasm target/wasm32v1-none/release/fungible_token_interface_example.wasm \
     --source testnet-dev \
     --network testnet \
     -- \
     --owner testnet-dev \
     --initial_supply "1000000000000000000000000"
   
   # Deploy quiz contract
   stellar contract deploy \
     --wasm target/wasm32v1-none/release/quiz.wasm \
     --source testnet-dev \
     --network testnet
   ```

3. **Update configuration:**
   ```bash
   # Update .env for testnet
   echo 'PUBLIC_STELLAR_NETWORK="TESTNET"
   PUBLIC_STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
   PUBLIC_STELLAR_RPC_URL="https://soroban-testnet.stellar.org"
   PUBLIC_STELLAR_HORIZON_URL="https://horizon-testnet.stellar.org"
   PUBLIC_TOKEN_CONTRACT_ID="<your-token-contract-id>"
   PUBLIC_QUIZ_CONTRACT_ID="<your-quiz-contract-id>"' > .env
   
   # Update environments.toml with your contract IDs
   ```

4. **Generate TypeScript clients:**
   ```bash
   STELLAR_SCAFFOLD_ENV=staging stellar scaffold watch --build-clients staging
   ```

5. **Start frontend:**
   ```bash
   # In another terminal
   npm run install:contracts
   npx vite
   ```

## Contract Usage

### Creating a Quiz Room

```bash
stellar contract invoke \
  --id <QUIZ_CONTRACT_ID> \
  --source testnet-dev \
  --network testnet \
  -- \
  init_pool_room \
  --room_id 123 \
  --host testnet-dev \
  --fee_token <TOKEN_CONTRACT_ID> \
  --entry_fee 100 \
  --host_fee_bps 100 \
  --prize_pool_bps 1000 \
  --first_place_pct 60 \
  --second_place_pct 25 \
  --third_place_pct 15
```

### Joining a Quiz Room

```bash
stellar contract invoke \
  --id <QUIZ_CONTRACT_ID> \
  --source testnet-dev \
  --network testnet \
  -- \
  join_room \
  --room_id 123 \
  --player testnet-dev \
  --screen_name "YourName" \
  --extras_amount 50
```

## Smart Contract Features

### Prize Modes

1. **Pool Split Mode**: Entry fees are pooled and distributed by percentage
2. **Asset Based Mode**: Host deposits specific prize assets (NFTs, tokens)

### Fee Distribution

- **Platform Fee**: 20% (fixed)
- **Host Fee**: 0-5% (configurable)
- **Prize Pool**: 0-25% (configurable for pool mode)
- **Charity**: Remainder (minimum 50%)

### Safety Features

- Input validation and overflow protection
- Screen name uniqueness enforcement
- Winner validation (must be players)
- Automatic remainder distribution to charity

## Development Notes

### Known Issues with Scaffold Stellar

If you encounter issues with the standard Scaffold Stellar workflow:

- Registry system may fail with `Error(Contract, #11)`
- Environment switching may not work as documented
- Use direct deployment method shown above as fallback

### Troubleshooting

- **MissingValue errors**: Usually indicates local/testnet network mismatch
- **Docker container issues**: Set `run-locally = false` in environments.toml
- **Client generation fails**: Use manual `stellar contract bindings typescript` command

## Contributing

1. Fork the repository
2. Create a feature branch
3. Test your changes on testnet
4. Submit a pull request

## License

[Add your license here]

## Links

- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Smart Contracts](https://developers.stellar.org/docs/build/smart-contracts)
- [Scaffold Stellar](https://github.com/AhaLabs/scaffold-stellar)
