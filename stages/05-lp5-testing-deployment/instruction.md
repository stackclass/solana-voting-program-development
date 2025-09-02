## Prerequisite Reading
- Anchor Testing Framework
- Solana Devnet Deployment
- TypeScript/JavaScript Basics

## Learning Objectives
- Write integration tests for Solana programs
- Deploy programs to devnet
- Test program functionality end-to-end
- Verify deployment success

## Task
1. Create basic test in `tests/voting-program.ts`:
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
  });
});
```

2. Deploy to devnet:
```bash
anchor deploy
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Test compilation | TypeScript compiles successfully |
| Test execution | All tests pass |
| Deployment | Program deploys to devnet |
| Program ID | Correct program ID in Anchor.toml |

## Notes
- Set ANCHOR_PROVIDER_URL to devnet
- Fund wallet with devnet SOL for deployment
- Verify program ID after deployment

## Reference Links
- https://www.anchor-lang.com/docs/testing
- https://solana.com/docs/cli/deploy
