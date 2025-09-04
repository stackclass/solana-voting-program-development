In this stage, you will install and configure the Solana development
environment, then generate a unique program ID for your voting smart contract.

## Your Tasks

1. **Install Development Tools**: Follow the official [Anchor Installation
   Documentation](https://www.anchor-lang.com/docs/installation) to install Rust
   programming language, Solana CLI tools, and Anchor framework.

2. **Configure Development Environment**: Set up your local development
   environment by following the [Anchor Quickstart
   Guide](https://www.anchor-lang.com/docs/quickstart/local)

3. **Generate Program ID**: Execute the command `anchor keys sync` to create a
   unique program ID for deployment

## Why This Step Is Important

The example code includes a default placeholder program ID. To deploy your smart
contract to the blockchain, you must replace this with a unique program ID. The
`anchor keys sync` command performs the following actions:

- Generates a new cryptographic keypair for your program
- Updates the [`declare_id!`
  macro](https://www.anchor-lang.com/docs/basics/program-structure#declare_id-macro)
  with your unique program identifier
- Ensures your contract is properly configured for deployment

## Tests

The tester will verify that your program ID has been successfully updated from
the default placeholder by executing:

```bash
grep "declare_id!" programs/voting-program/src/lib.rs
```

 ## Notes

- Every Solana program requires a unique identifier for on-chain deployment
- The `anchor keys sync` command generates the keypair file in the `target`
  directory
- After synchronization, your program is ready for building and deployment
- Make sure you are working within a properly configured local development
  environment
