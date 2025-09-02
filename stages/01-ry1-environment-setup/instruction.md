## Prerequisite Reading
- Solana Documentation: Getting Started
- Anchor Framework: Installation Guide
- Rust Programming Language Basics

## Learning Objectives
- Install required development tools (Rust, Solana CLI, Anchor)
- Create Solana development wallet
- Initialize Anchor project structure
- Verify environment configuration

## Task
1. Install Rust toolchain:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

2. Install Solana CLI:
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.10.32/install)"
```

3. Install Anchor framework:
```bash
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

4. Create new project:
```bash
anchor init solana-voting
cd solana-voting
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| `solana --version` | Returns 1.10.x version |
| `anchor --version` | Returns 0.24.x version |
| `anchor build` | Compiles successfully |
| Project structure | Contains programs/, tests/, migrations/ |

## Notes
- Ensure Rust is properly installed with rustup
- Set Solana to devnet: `solana config set --url devnet`
- Create test wallet if needed: `solana-keygen new`

## Reference Links
- https://solana.com/docs
- https://www.anchor-lang.com/docs
- https://rustup.rs
