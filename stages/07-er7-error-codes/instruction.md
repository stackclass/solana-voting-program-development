Robust error handling is crucial for user-friendly smart contracts. In this
stage, you'll define custom error codes that provide clear feedback when
validation fails, making your voting program more professional and easier to
debug.

## Prerequisite Reading

To understand error handling in Anchor, review:

- **Anchor Error Handling**: Learn how Anchor manages errors and provides
  user-friendly messages. The [Anchor Error
  Documentation](https://www.anchor-lang.com/docs/features/errors) covers the
  error handling system.
- **Custom Error Codes**: Understand how to define and use custom error types.
  Read about [Error Code
  Attribute](https://docs.rs/anchor-lang/latest/anchor_lang/attr.error_code.html)
  in Anchor's documentation.
- **Condition Checking Patterns**: Learn about common validation patterns using
  `require!` macros. Review [Anchor Require
  Macro](https://www.anchor-lang.com/docs/features/errors#require) for
  validation examples.

## Implement Custom Error Codes

Add the following error enum definition:

```rust
#[error_code]
pub enum VotingError {
    #[msg("Invalid option index")]
    InvalidOptionIndex,
}
```

## Understanding Error Handling in Anchor

Let's examine the error handling components:

- **`#[error_code]`**: This attribute macro tells Anchor that this enum
  represents custom error codes for your program. It automatically generates the
  necessary boilerplate.

- **`pub enum VotingError`**: Defines a set of possible errors that can occur in
  your voting program. Enums are ideal for error types because they can
  represent multiple error cases.

- **`#[msg("Invalid option index")]`**: Provides a user-friendly error message
  that will be displayed when this error occurs. This is much better than
  cryptic error codes.

## Why Custom Error Codes Matter

Custom error codes provide several benefits:

1. **User Experience**: Clear, descriptive messages help users understand what
   went wrong
2. **Debugging**: Specific error types make debugging easier
3. **Testing**: Well-defined errors enable comprehensive test coverage
4. **Maintenance**: Organized error handling makes code easier to maintain and
   extend

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Error enum compiles | No syntax errors | Ensures proper Rust syntax |
| `#[error_code]` attribute present | Proper annotation | Confirms Anchor recognizes this as error enum |
| Error message defined | Clear user feedback | Validates descriptive error messaging |

## Notes

- The `#[error_code]` attribute is required for Anchor to properly handle your custom errors
- `#[msg("...")]` provides user-friendly messages that will be displayed to users
- Error codes should be specific and descriptive to aid debugging
- You can add more error variants as your program grows in complexity
