# Coding Conventions

This document outlines coding standards and best practices for the spire-board project.

## General Principles

### 1. Simplicity First

- Avoid over-engineering
- Don't add features beyond what's requested
- Keep solutions focused and minimal

### 2. Code Quality

- Write self-documenting code with clear variable/function names
- Add comments only where logic isn't self-evident
- Follow existing patterns in the codebase

### 3. Security

- Prevent common vulnerabilities (XSS, SQL injection, command injection)
- Validate input at system boundaries
- Use parameterized queries for databases
- Sanitize user input appropriately

## Rust (Backend) Conventions

### Naming Conventions

- **Files:** `snake_case.rs`
- **Functions:** `snake_case`
- **Structs/Enums:** `PascalCase`
- **Constants:** `SCREAMING_SNAKE_CASE`
- **Modules:** `snake_case`

**Examples:**
```rust
// File: user_service.rs
const MAX_RETRIES: u32 = 3;

struct UserProfile {
    user_id: String,
    display_name: String,
}

fn create_user_profile(name: String) -> UserProfile {
    // ...
}
```

### Code Organization

- **One module per file**
- **Group related functionality in modules**
- **Use `mod.rs` for module exports**
- **Keep handlers, models, and services separate**

**Example structure:**
```
backend/src/
├── main.rs
├── handlers/
│   ├── mod.rs
│   └── user_handler.rs
├── models/
│   ├── mod.rs
│   └── user.rs
└── services/
    ├── mod.rs
    └── user_service.rs
```

### Error Handling

- **Use `Result<T, E>` for recoverable errors**
- **Use custom error types with `thiserror`**
- **Return errors, don't panic in production code**
- **Use `?` operator for error propagation**

**Example:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found: {0}")]
    NotFound(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

fn get_user(id: &str) -> Result<User, UserError> {
    let user = query_user(id)?;  // Propagate error
    Ok(user)
}
```

### Testing

- **Unit tests in same file:** `#[cfg(test)] mod tests`
- **Integration tests in `tests/` directory**
- **Use `cargo test` for running tests**

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let user = create_user("Alice".to_string());
        assert_eq!(user.display_name, "Alice");
    }
}
```

### Documentation

- **Add doc comments for public APIs**
- **Use `///` for function/struct documentation**
- **Use `//!` for module documentation**

**Example:**
```rust
/// Creates a new user profile with the given name.
///
/// # Arguments
///
/// * `name` - The display name for the user
///
/// # Returns
///
/// A `UserProfile` instance
pub fn create_user_profile(name: String) -> UserProfile {
    // ...
}
```

## PureScript (Frontend) Conventions

### Naming Conventions

- **Files:** `PascalCase.purs`
- **Functions:** `camelCase`
- **Types:** `PascalCase`
- **Type variables:** lowercase `a`, `b`, etc.
- **Modules:** `PascalCase.Nested.Module`

**Examples:**
```purescript
-- File: UserProfile.purs
module App.Component.UserProfile where

type UserId = String

data UserProfile = UserProfile
  { userId :: UserId
  , displayName :: String
  }

createUserProfile :: String -> UserProfile
createUserProfile name = UserProfile { userId: "", displayName: name }
```

### Code Organization

- **One module per file**
- **Module name matches file path**
- **Group components by feature**
- **Separate API client from components**
- **Use Capability pattern for dependency injection**
- **Follow [Real World Halogen](https://thomashoneyman.com/guides/real-world-halogen/) architecture patterns**

**Example structure:**
```
frontend/src/
├── Main.purs
├── Component/
│   ├── UserProfile.purs
│   └── Dashboard.purs
├── Data/
│   └── User.purs
├── Api/
│   └── User.purs
└── Capability/
    └── Resource.purs
```

### Error Handling

- **Use `Either` for operations that can fail**
- **Use `Maybe` for optional values**
- **Use `Aff` for asynchronous operations**
- **Handle errors at component boundaries**

**Example:**
```purescript
import Data.Either (Either(..))
import Data.Maybe (Maybe(..))
import Effect.Aff (Aff)

type ApiError = String

fetchUser :: UserId -> Aff (Either ApiError User)
fetchUser id = do
  response <- makeRequest id
  pure $ case response of
    Success user -> Right user
    Failure err -> Left err
```

### Testing

- **Tests in `test/` directory**
- **Use `spago test` for running tests**
- **Use QuickCheck for property-based testing**

**Example:**
```purescript
module Test.Main where

import Prelude
import Test.Unit (suite, test)
import Test.Unit.Assert as Assert

main = do
  suite "UserProfile" do
    test "creates user with name" do
      let user = createUserProfile "Alice"
      Assert.equal "Alice" user.displayName
```

### Type Safety

- **Leverage PureScript's type system**
- **Use newtypes for domain types**
- **Avoid `unsafe` functions in production**

**Example:**
```purescript
newtype UserId = UserId String

derive instance newtypeUserId :: Newtype UserId _
derive newtype instance eqUserId :: Eq UserId
derive newtype instance showUserId :: Show UserId
```

## Testing Requirements

### General Guidelines

- Write tests for new functionality
- Maintain or improve code coverage
- Run full test suite before committing
- Include both unit and integration tests where appropriate

### Test Naming

**Rust:**
```rust
#[test]
fn test_user_creation_succeeds() { }

#[test]
fn test_user_creation_fails_with_invalid_input() { }
```

**PureScript:**
```purescript
test "user creation succeeds" do
test "user creation fails with invalid input" do
```

## Security Best Practices

### Input Validation

- Validate at system boundaries
- Sanitize user input
- Use allowlists over denylists

### Authentication & Authorization

- Follow established auth patterns
- Never hardcode credentials
- Use environment variables for secrets

### Data Protection

- Encrypt sensitive data
- Use secure communication protocols
- Follow principle of least privilege

## Code Review Standards

Before submitting code for review:

1. **All tests pass**
2. **Code is formatted** (`cargo fmt` / `purs-tidy`)
3. **No linter warnings** (`cargo clippy`)
4. **Documentation is updated**
5. **No debug code or console.log** statements
6. **No commented-out code**
7. **Security vulnerabilities addressed**

## Version Control

### What to Commit

- Source code
- Configuration templates (`.env.example`)
- Documentation
- Tests
- Build scripts

### What NOT to Commit

- Build outputs (`target/`, `output/`)
- Dependencies (`node_modules/`, `.spago/`)
- Environment files (`.env`)
- IDE-specific files (`.vscode/`, `.idea/`)
- Temporary files
- Database files

See `.gitignore` for complete list.

## Performance Considerations

### Backend (Rust)

- Use `cargo build --release` for production
- Avoid unnecessary cloning
- Use references where possible
- Profile before optimizing

### Frontend (PureScript)

- Minimize component re-renders
- Use lazy evaluation where appropriate
- Optimize bundle size
- Profile in browser DevTools

## Accessibility

### Frontend Components

- Use semantic HTML
- Include ARIA labels where needed
- Ensure keyboard navigation works
- Test with screen readers

## Internationalization (i18n)

- Keep user-facing strings separate
- Use i18n library for translations
- Support RTL languages if needed

## Documentation Standards

### Code Comments

**Good comments explain WHY, not WHAT:**
```rust
// Bad
let x = 5; // Set x to 5

// Good
// Retry limit based on network latency testing
const MAX_RETRIES: u32 = 5;
```

### Documentation Updates

Update documentation when:
- Adding new features
- Changing APIs
- Modifying architecture
- Discovering common issues

## References

For more detailed patterns and examples:
- [Real World Halogen Guide](https://thomashoneyman.com/guides/real-world-halogen/) - PureScript/Halogen patterns
- [Real World Halogen Example](https://github.com/thomashoneyman/purescript-halogen-realworld) - Reference implementation
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - Rust best practices
