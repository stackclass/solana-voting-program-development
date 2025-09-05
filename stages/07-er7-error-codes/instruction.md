## Prerequisite Reading
- Anchor Error Handling
- Custom Error Codes
- Condition Checking Patterns

## Learning Objectives
- Define custom error types
- Implement proper error messages
- Set up error code enum

## Task
1. Add the error enum to your program:

```rust
#[error_code]
pub enum VotingError {
    #[msg("Invalid option index")]
    InvalidOptionIndex,
}
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Error enum compiles | No syntax errors |
- #[error_code] attribute present | Proper annotation |
- Error message defined | Clear user feedback |

## Notes
- `#[error_code]` attribute required
- `#[msg("...")]` provides user-friendly error messages
- Error codes help with debugging and testing

## Reference Links
- https://www.anchor-lang.com/docs/errors
- https://docs.rs/anchor-lang/latest/anchor_lang/attr.error_code.html
