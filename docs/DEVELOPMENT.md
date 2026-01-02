# Development Guide

This guide covers day-to-day development workflows, commands, and best practices.

## Git Workflow

### Branch Strategy

**Main Development Branch:** TBD (typically `main` or `master`)
**Feature Branches:** Use pattern `claude/feature-name-{sessionId}`

### Branch Guidelines

1. **Never push directly to the main branch** without explicit permission
2. **Always work on feature branches** named with the `claude/` prefix
3. **Branch naming convention:** `claude/descriptive-name-{sessionId}`
4. **Keep branches focused:** One feature or fix per branch

### Making Changes

**1. Before starting work:**
```bash
git fetch origin
git checkout -b claude/feature-name-{sessionId}
```

**2. During development:**
- Make focused, logical commits
- Write clear commit messages (see conventions below)
- Commit related changes together

**3. Before pushing:**
```bash
git status  # Review changes
git diff    # Inspect modifications
```

**4. Pushing changes:**
```bash
git push -u origin claude/feature-name-{sessionId}
```

**Critical:** Branch must start with `claude/` and end with session ID
- Retry on network failures: up to 4 times with exponential backoff (2s, 4s, 8s, 16s)

### Commit Message Conventions

Follow these patterns:

- **Features:** `feat: add user authentication system`
- **Bug fixes:** `fix: resolve memory leak in data processing`
- **Refactoring:** `refactor: simplify error handling logic`
- **Documentation:** `docs: update API documentation`
- **Tests:** `test: add unit tests for validation module`
- **Chores:** `chore: update dependencies`

**Format:**
```
<type>: <short description>

<optional detailed description>

<optional breaking changes note>
```

### Pull Request Process

**1. Create descriptive PRs with:**
- Clear title summarizing the change
- Detailed description of what changed and why
- Test plan or verification steps
- Reference to related issues

**2. PR Description Template:**
```markdown
## Summary
- Brief overview of changes
- Why these changes were needed

## Changes Made
- List specific modifications
- Files affected

## Test Plan
- [ ] Steps to verify the changes
- [ ] Edge cases considered
- [ ] Existing tests still pass
```

## Development Commands

### Turborepo (Recommended)

Run commands across all workspaces:

```bash
# Run all development servers (backend + frontend)
npm run dev

# Build all workspaces
npm run build

# Run all tests
npm run test

# Run linter across all workspaces
npm run lint

# Format code across all workspaces
npm run format

# Clean all build outputs
npm run clean
```

### Backend (Rust)

**From root:**
```bash
npm run dev --filter=backend
npm run build --filter=backend
npm run test --filter=backend
```

**From backend directory:**
```bash
cd backend

# Development mode (with auto-reload)
cargo watch -x run

# Production mode
cargo run --release

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt

# Check without building
cargo check
```

### Frontend (PureScript)

**From root:**
```bash
npm run dev --filter=frontend
npm run build --filter=frontend
npm run test --filter=frontend
```

**From frontend directory:**
```bash
cd frontend

# Development mode (with auto-reload)
spago build --watch

# Build for production
spago build

# Run tests
spago test

# Start REPL
spago repl

# Format code
purs-tidy format-in-place 'src/**/*.purs'

# Clean build outputs
rm -rf output .spago
```

### Database Operations

**Run migrations:**
```bash
cd backend
cargo sqlx migrate run
```

**Create new migration:**
```bash
cd backend
cargo sqlx migrate add <migration-name>
```

**Revert last migration:**
```bash
cd backend
cargo sqlx migrate revert
```

**Connect to database:**
```bash
docker compose exec postgres psql -U postgres -d spire_board
```

### Docker Operations

**Start all services:**
```bash
docker compose up -d
```

**Start specific service:**
```bash
docker compose up -d postgres
```

**View logs:**
```bash
docker compose logs -f
docker compose logs -f postgres  # specific service
```

**Stop all services:**
```bash
docker compose down
```

**Rebuild containers:**
```bash
docker compose up -d --build
```

**Remove volumes (WARNING: deletes data):**
```bash
docker compose down -v
```

**List running containers:**
```bash
docker compose ps
```

## Development Best Practices

### Before Making Changes

1. **Always read files before modifying them**
   - Use Read tool to understand existing code
   - Never propose changes to unread code
   - Understand context and patterns

2. **Research first**
   - Use Explore agent for codebase exploration
   - Search for existing patterns and similar implementations
   - Check for related functionality

### Making Changes

1. **Prefer editing over creating**
   - Always edit existing files when possible
   - Only create new files when absolutely necessary
   - Don't create documentation unless requested

2. **Minimize changes**
   - Only modify what's needed for the task
   - Don't refactor surrounding code unless asked
   - Don't add "improvements" beyond the request
   - Avoid adding extra error handling for impossible scenarios

3. **Follow existing patterns**
   - Match the style of surrounding code
   - Use established patterns in the codebase
   - Don't introduce new patterns without good reason

### Code Review Checklist

Before committing, verify:
- [ ] All requested functionality is implemented
- [ ] No unrelated changes included
- [ ] Security vulnerabilities addressed
- [ ] Tests pass
- [ ] Code follows project conventions
- [ ] No backwards-compatibility hacks
- [ ] Unused code removed completely (not commented out)

## Debugging

### Backend Debugging

**Enable detailed logging:**
```bash
RUST_LOG=debug cargo run
```

**Use tracing for specific modules:**
```bash
RUST_LOG=spire_board_backend=debug,sqlx=info cargo run
```

### Frontend Debugging

**Check build output:**
```bash
spago build --verbose
```

**Use REPL for testing:**
```bash
spago repl
> import Prelude
> import Effect.Console (log)
```

### Database Debugging

**Check PostgreSQL logs:**
```bash
docker compose logs postgres
```

**Query database directly:**
```bash
docker compose exec postgres psql -U postgres -d spire_board
\dt  # List tables
\d table_name  # Describe table
```

## Testing

### Backend Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests with specific filter
cargo test --test integration_test
```

### Frontend Tests

```bash
# Run all tests
spago test

# Run tests with specific pattern
spago test --match "Pattern"
```

## Performance Monitoring

### Backend

- Use `cargo build --release` for production builds
- Profile with `cargo flamegraph` for performance issues
- Monitor with `RUST_LOG=info` for production logging

### Frontend

- Use `spago build` with optimizations for production
- Check bundle size in `output/` directory

## Common Issues

See [SETUP.md](./SETUP.md#troubleshooting) for setup-related issues.

### Hot Reload Not Working

**Backend:**
```bash
cargo install cargo-watch  # Install if missing
cargo watch -x run
```

**Frontend:**
```bash
spago build --watch
```

### Tests Failing After Changes

1. Clean build outputs
2. Rebuild from scratch
3. Check for breaking changes in dependencies

## Getting Help

- Check existing documentation
- Search codebase for similar implementations
- Review git history for context
- Ask specific questions with context
