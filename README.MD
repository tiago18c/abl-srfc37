# ABL sRFC-37 Thawing Gate Program

A Solana smart contract implementation of a thawing gate program that integrates with sRFC-37, enabling permissionless thawing of token accounts as defined by a gating program.

## Overview

This program implements a thawing gate that controls when token accounts can be thawed (unfrozen) in a permissionless manner. It supports three different modes of operation and integrates with the EBALTS (Efficient Block/Allow List Token Standard) interface to provide flexible token account management.

## Features

- **Three Operation Modes**: Allow, AllowWithPermissionlessEOAs, and Block
- **Whitelist/Blacklist Management**: Add and remove wallets from the list
- **sRFC-37 Integration**: Implements the required interface for permissionless thawing
- **PDA-based List Configuration**: Uses Program Derived Addresses for management of multiple lists

## Program ID

```
CMKdNTip9w21wNxfwN6J9pdRGMosNDx9KjpKMVuLPQVu
```

## Architecture

### Core Components

#### State Accounts

- **`ListConfig`**: Main configuration account containing authority, seed, mode, and bump. Represents an allow / block list.
- **`ABWallet`**: Individual wallet accounts that represent allowlisted/blocklisted addresses

#### Operation Modes

1. **`Allow`**: Only wallets that have been explicitly allowlisted can thaw
2. **`AllowWithPermissionlessEOAs`**: All wallets that can sign can thaw, otherwise requires allowlisting (for PDAs)
3. **`Block`**: Only wallets that have been blocklisted cannot thaw

### Instructions

#### Configuration Instructions

- **`initialize_list_config`**: Initialize the main list configuration
- **`set_list_mode`**: Change the operation mode
- **`add_wallet_to_list`**: Add a wallet to the allowlist/blocklist
- **`remove_wallet_from_list`**: Remove a wallet from the list
- **`set_extra_metas_thaw`**: Set additional metadata for thawing operations

#### sRFC-37 Interface Instructions

- **`can_thaw_permissionless`**: Main gating function that determines if thawing is allowed
- **`can_freeze_permissionless`**: Returns error (not supported in this implementation)

## Usage

### Prerequisites

- Solana CLI tools
- Anchor Framework
- Node.js and Yarn

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd abl-srfc37
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build
```

### Deployment

1. Configure your Solana cluster in `Anchor.toml`
2. Deploy the program:
```bash
anchor deploy
```

### Testing

Run the test suite:
```bash
yarn test
```

## Integration with sRFC-37

This program implements the sRFC-37 interface by providing the `can_thaw_permissionless` instruction that:

1. Validates the calling context
2. Checks the current operation mode
3. Verifies wallet permissions based on the mode
4. Returns success or failure to allow/deny thawing


## Development

### Project Structure

```
programs/abl-srfc37/
├── src/
│   ├── lib.rs              # Main program entry point
│   ├── state.rs            # Account structures and enums
│   ├── errors.rs           # Custom error definitions
│   ├── constants.rs        # Program constants
│   └── instructions/       # Individual instruction implementations
│       ├── mod.rs
│       ├── initialize_list_config.rs
│       ├── add_wallet_to_list.rs
│       ├── remove_wallet_from_list.rs
│       ├── set_list_mode.rs
│       ├── can_thaw_permissionless.rs
│       ├── can_freeze_permissionless.rs
│       └── set_extra_metas_thaw.rs
├── Cargo.toml
└── Xargo.toml
```
