In this final stage, you'll complete your voting program by writing
comprehensive integration tests and deploying it to the Solana devnet. Testing
ensures your program works correctly, and deployment makes it available on a
real blockchain network.

## Prerequisite Reading

To understand testing and deployment, review:

- **Anchor Testing Framework**: Learn how Anchor simplifies Solana program
  testing. The [Anchor Testing
  Documentation](https://www.anchor-lang.com/docs/testing) covers the testing
  framework and patterns.
- **Solana Devnet Deployment**: Understand the deployment process and devnet
  environment. Read [Deploying Programs
  Documentation](https://solana.com/docs/programs/deploying) for deployment
  instructions.
- **Anchor Deployment**: Learn how to deploy programs using Anchor's built-in
  deployment tools. The [Anchor Deployment
  Documentation](https://www.anchor-lang.com/docs/quickstart/local#deploy-to-devnet)
  covers deployment to devnet using Anchor.
- **TypeScript/JavaScript Basics**: Since tests are written in TypeScript,
  review basic concepts if needed. The [TypeScript
  Handbook](https://www.typescriptlang.org/docs/handbook/) provides a good
  foundation.

## Create Comprehensive Tests

Create or update `tests/voting-program.ts` with the following test:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingProgram } from "../target/types/voting_program";

describe("voting-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VotingProgram as Program<VotingProgram>;

  it("Creates a proposal", async () => {
    const [proposalPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("proposal"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeProposal("Test Proposal", ["Option A", "Option B"])
      .accounts({
        proposal: proposalPDA,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const proposal = await program.account.proposal.fetch(proposalPDA);
    console.log("Proposal created:", proposal.title);

    // Verify proposal data
    expect(proposal.title).toEqual("Test Proposal");
    expect(proposal.options).toEqual(["Option A", "Option B"]);
    expect(proposal.votes).toEqual([0, 0]);
  });

  it("Allows voting on a proposal", async () => {
    const [proposalPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("proposal"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    // Vote for option 0
    await program.methods
      .vote(new anchor.BN(0))
      .accounts({
        proposal: proposalPDA,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const proposal = await program.account.proposal.fetch(proposalPDA);
    expect(proposal.votes).toEqual([1, 0]);
  });

  it("Prevents voting with invalid option index", async () => {
    const [proposalPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("proposal"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    // Try to vote for invalid option index 99
    try {
      await program.methods
        .vote(new anchor.BN(99))
        .accounts({
          proposal: proposalPDA,
          user: provider.wallet.publicKey,
        })
        .rpc();
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.message).toContain("Invalid option index");
    }
  });
});
```

## Deploy to Devnet

### 1. Configure for Devnet

Update your `Anchor.toml` to use devnet:

```toml
[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[programs.devnet]
voting_program = "YOUR_PROGRAM_ID_HERE"
```

### 2. Deploy Your Program

Execute the deployment command:

```bash
anchor deploy
```

## Understanding the Testing Process

The tests verify several key aspects:

1. **Proposal Creation**: Tests that proposals can be created with proper data
2. **Data Storage**: Verifies that title, options, and initial votes are stored correctly
3. **Voting Functionality**: Tests that votes can be cast and counted properly
4. **Error Handling**: Confirms that invalid votes are properly rejected
5. **PDA Usage**: Verifies that Program Derived Addresses work correctly

## Understanding Deployment

The deployment process:

1. **Builds** your program for the target network (devnet)
2. **Uploads** the program to the Solana devnet
3. **Verifies** the deployment was successful
4. **Updates** your local configuration with the deployed program ID

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Test compilation | TypeScript compiles successfully | Ensures test syntax is correct |
| Test execution | All tests pass | Validates program functionality |
| Deployment | Program deploys to devnet | Confirms successful deployment |
| Program ID | Correct program ID in Anchor.toml | Ensures proper configuration |

## Notes

- Set `ANCHOR_PROVIDER_URL` to devnet URL: `https://api.devnet.solana.com`
- Fund your wallet with devnet SOL using `solana airdrop 2` command
- Verify your program ID in `Anchor.toml` matches the deployed program
- Test thoroughly before deployment to avoid unnecessary costs

## Congratulations!

You've now completed the full voting program development lifecycle - from
environment setup to deployment. You've learned:

- Solana account structure design
- PDA initialization and management
- Secure voting logic implementation
- Comprehensive testing strategies
- Production deployment procedures

Your voting program is now ready for real-world use on the Solana blockchain!
