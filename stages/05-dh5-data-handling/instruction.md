## Prerequisite Reading
- Anchor Context Usage
- Rust Borrowing and Mutability
- Data Validation Patterns

## Learning Objectives
- Implement account data initialization
- Handle string and vector data safely
- Store PDA bump seed for reference

## Task
1. Complete the initialize_proposal function:

```rust
pub fn initialize_proposal(
    ctx: Context<InitializeProposal>,
    title: String,
    options: Vec<String>
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    proposal.title = title;
    proposal.options = options.clone();
    proposal.votes = vec![0; options.len()];
    proposal.bump = *ctx.bumps.get("proposal").unwrap();
    Ok(())
}
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Function compiles | No syntax errors |
| Data assignment | Title and options stored correctly |
| Votes initialization | All options start with 0 votes |
| Bump storage | PDA bump seed properly stored |

## Notes
- Use `&mut` for mutable account reference
- Clone options to avoid ownership issues
- Initialize votes vector with zeros
- Store bump seed from context

## Reference Links
- https://www.anchor-lang.com/docs/context
- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
