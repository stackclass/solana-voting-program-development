## Prerequisite Reading
- Program Derived Addresses (PDAs)
- Anchor Space Calculation
- Account Rent Mechanics

## Learning Objectives
- Understand PDA concept and usage
- Calculate proper account space
- Implement secure PDA initialization

## Task
1. Focus on the space calculation and PDA initialization in the context:

```rust
#[account(
    init,
    payer = user,
    space = 8 + 4 + title.len() + 4 + (options.iter().map(|opt| 4 + opt.len()).sum::<usize>()) + 4 + (options.len() * 4) + 1,
    seeds = [b"proposal", user.key().as_ref()],
    bump
)]
pub proposal: Account<'info, Proposal>,
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Space calculation | Proper account size allocation |
| PDA seeds | Correct seed structure |
| Bump storage | Bump seed properly handled |

## Notes
- `8` bytes for Anchor discriminator
- `4 + title.len()` for String storage
- `4 + options.*` for Vec<String> storage
- `4 + options.len() * 4` for Vec<u32> storage
- `1` byte for bump field

## Reference Links
- https://www.anchor-lang.com/docs/space
- https://solana.com/docs/core/pdas
