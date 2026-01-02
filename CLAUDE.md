# CLAUDE.md - AI Assistant Guide for spire-board

This document provides comprehensive guidance for AI assistants working on the spire-board codebase.

## Repository Overview

**Project Name:** spire-board
**Repository:** nesso-pfl/spire-board
**Current Status:** Initial setup phase

### Project Description

spire-board is a web application with a clear separation between backend API and frontend client:
- **Backend:** REST API server with WebSocket support
- **Frontend:** Modern web UI for user interaction
- **Architecture:** Client-server model with separate codebases

## Codebase Structure

### Directory Organization

```
spire-board/
├── backend/          # Rust API server
│   ├── src/          # Rust source code
│   ├── migrations/   # Database migrations
│   ├── tests/        # Backend tests
│   └── Cargo.toml    # Rust dependencies
├── frontend/         # PureScript web client
│   ├── src/          # PureScript source code
│   ├── test/         # Frontend tests
│   ├── spago.yaml    # Spago configuration
│   └── packages.dhall # PureScript packages
├── docker/           # Docker configuration
│   └── docker-compose.yml
├── docs/             # Documentation
└── CLAUDE.md         # AI assistant guide
```

### Key Files and Their Purposes

**Backend (Rust):**
- `backend/Cargo.toml` - Rust dependencies and project metadata
- `backend/src/main.rs` - API server entry point
- `backend/migrations/` - Database schema migrations (Drizzle)

**Frontend (PureScript):**
- `frontend/spago.yaml` - Spago build configuration
- `frontend/packages.dhall` - PureScript package dependencies
- `frontend/src/Main.purs` - Frontend application entry point

**Infrastructure:**
- `docker-compose.yml` - Local development environment
- `.env` - Environment variables (not committed to git)

## Development Workflows

### Branch Strategy

**Main Development Branch:** TBD (typically `main` or `master`)
**Feature Branches:** Use pattern `claude/feature-name-{sessionId}`

#### Branch Guidelines

1. **Never push directly to the main branch** without explicit permission
2. **Always work on feature branches** named with the `claude/` prefix
3. **Branch naming convention:** `claude/descriptive-name-{sessionId}`
4. **Keep branches focused:** One feature or fix per branch

### Git Workflow

#### Making Changes

1. **Before starting work:**
   ```bash
   git fetch origin
   git checkout -b claude/feature-name-{sessionId}
   ```

2. **During development:**
   - Make focused, logical commits
   - Write clear commit messages (see conventions below)
   - Commit related changes together

3. **Before pushing:**
   ```bash
   git status  # Review changes
   git diff    # Inspect modifications
   ```

4. **Pushing changes:**
   ```bash
   git push -u origin claude/feature-name-{sessionId}
   ```
   - **Critical:** Branch must start with `claude/` and end with session ID
   - Retry on network failures: up to 4 times with exponential backoff (2s, 4s, 8s, 16s)

#### Commit Message Conventions

Follow these patterns for commit messages:

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

1. **Create descriptive PRs** with:
   - Clear title summarizing the change
   - Detailed description of what changed and why
   - Test plan or verification steps
   - Reference to related issues

2. **PR Description Template:**
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

## Coding Conventions

### General Principles

1. **Simplicity First**
   - Avoid over-engineering
   - Don't add features beyond what's requested
   - Keep solutions focused and minimal

2. **Code Quality**
   - Write self-documenting code with clear variable/function names
   - Add comments only where logic isn't self-evident
   - Follow existing patterns in the codebase

3. **Security**
   - Prevent common vulnerabilities (XSS, SQL injection, command injection)
   - Validate input at system boundaries
   - Use parameterized queries for databases
   - Sanitize user input appropriately

### Language-Specific Conventions

#### Rust (Backend)

**Naming Conventions:**
- Files: `snake_case.rs`
- Functions: `snake_case`
- Structs/Enums: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

**Code Organization:**
- One module per file
- Group related functionality in modules
- Use `mod.rs` for module exports
- Keep handlers, models, and services separate

**Error Handling:**
- Use `Result<T, E>` for recoverable errors
- Use custom error types with `thiserror`
- Return errors, don't panic in production code
- Use `?` operator for error propagation

**Testing:**
- Unit tests in same file: `#[cfg(test)] mod tests`
- Integration tests in `tests/` directory
- Use `cargo test` for running tests

#### PureScript (Frontend)

**Naming Conventions:**
- Files: `PascalCase.purs`
- Functions: `camelCase`
- Types: `PascalCase`
- Type variables: lowercase `a`, `b`, etc.
- Modules: `PascalCase.Nested.Module`

**Code Organization:**
- One module per file
- Module name matches file path
- Group components by feature
- Separate API client from components

**Error Handling:**
- Use `Either` for operations that can fail
- Use `Maybe` for optional values
- Use `Aff` for asynchronous operations
- Handle errors at component boundaries

**Testing:**
- Tests in `test/` directory
- Use `spago test` for running tests
- Use QuickCheck for property-based testing

### Testing Requirements

- Write tests for new functionality
- Maintain or improve code coverage
- Run full test suite before committing
- Include both unit and integration tests where appropriate

## AI Assistant Guidelines

### Before Making Changes

1. **Always read files before modifying them**
   - Use Read tool to understand existing code
   - Never propose changes to unread code
   - Understand context and patterns

2. **Research first**
   - Use Explore agent for codebase exploration
   - Search for existing patterns and similar implementations
   - Check for related functionality

### Task Management

1. **Use TodoWrite tool** for multi-step tasks:
   - Break complex work into steps
   - Track progress in real-time
   - Mark tasks complete immediately after finishing

2. **One task in progress at a time**
   - Update status as you work
   - Complete current task before starting new ones

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

### Communication

1. **Be concise and clear**
   - Provide direct technical information
   - Focus on facts and problem-solving
   - Avoid unnecessary praise or superlatives

2. **Show your work**
   - Explain what you're doing and why
   - Reference specific files and line numbers: `file_path:line_number`
   - Provide context for decisions

## Common Tasks

### Project Setup

**Initial setup:**
```bash
# 1. Clone repository
git clone <repository-url>
cd spire-board

# 2. Set up environment variables
cp .env.example .env
# Edit .env with your configuration

# 3. Start Docker services
docker-compose up -d

# 4. Set up backend
cd backend
cargo build
cargo run migrate  # Run database migrations

# 5. Set up frontend
cd ../frontend
spago build
```

### Running the Project

**Backend (Rust/Actix):**
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
```

**Frontend (PureScript/Halogen):**
```bash
cd frontend

# Development mode (with auto-reload)
spago build --watch

# Build for production
spago build

# Run tests
spago test

# Format code
purs-tidy format-in-place 'src/**/*.purs'
```

**Database:**
```bash
# Run migrations
cd backend
cargo run migrate

# Create new migration
cargo run migrate:create <migration-name>

# Rollback migration
cargo run migrate:rollback
```

**Docker:**
```bash
# Start all services
docker-compose up -d

# Start specific service
docker-compose up -d postgres

# View logs
docker-compose logs -f

# Stop all services
docker-compose down

# Rebuild containers
docker-compose up -d --build
```

### Debugging

**Best practices:**
- Use appropriate debugging tools for the stack
- Check logs and error messages
- Reproduce issues before fixing
- Add tests to prevent regression

## Technology Stack

### Backend

**Language:** Rust (latest stable)

**Web Framework:**
- [Actix Web](https://actix.rs/) - High-performance web framework
- WebSocket support via Actix

**Database:**
- PostgreSQL 17
- [Drizzle ORM](https://orm.drizzle.team/) - TypeScript ORM for database access

**Key Dependencies:**
- `actix-web` - Web framework
- `actix-ws` - WebSocket support
- Database driver for PostgreSQL

### Frontend

**Language:** PureScript

**Build Tool:**
- [Spago](https://github.com/purescript/spago) - PureScript package manager and build tool

**UI Framework:**
- [Halogen](https://github.com/purescript-halogen/purescript-halogen) - Declarative UI framework

**Key Dependencies:**
- `purescript-halogen` - UI framework
- `purescript-affjax` - HTTP client (for API calls)
- `purescript-routing` - Client-side routing

### Development Environment

**Containerization:**
- Docker & Docker Compose for local development
- PostgreSQL 17 container
- Backend development container (optional)
- Frontend development container (optional)

**Development Tools:**
- Rust toolchain (rustc, cargo)
- PureScript compiler
- Spago
- Docker & Docker Compose

## Architecture Patterns

### Overall Architecture

**Client-Server Separation:**
- Backend and frontend are completely separate applications
- Communication via REST API and WebSocket
- Backend serves as API-only server (no server-side rendering)

**Backend Patterns:**
- RESTful API design
- WebSocket for real-time communication
- Actix Web actors for concurrency
- Drizzle ORM for type-safe database queries

**Frontend Patterns:**
- Component-based architecture (Halogen)
- Functional programming paradigms (PureScript)
- Type-safe API client
- Client-side routing

**Data Flow:**
1. Frontend makes HTTP/WebSocket requests to backend
2. Backend processes requests, queries database via Drizzle
3. Backend returns JSON responses
4. Frontend updates UI based on responses

## Dependencies

**Track important dependencies:**
- Core runtime dependencies
- Development dependencies
- Version constraints and compatibility notes
- Known issues or workarounds

## Environment Setup

### Required Tools and Versions

**Backend Development:**
- Rust (latest stable) - Install via [rustup](https://rustup.rs/)
- Cargo (comes with Rust)
- PostgreSQL client tools (for local development)

**Frontend Development:**
- Node.js (LTS version) - Required for PureScript tooling
- PureScript compiler - Install via npm: `npm install -g purescript`
- Spago - Install via npm: `npm install -g spago`

**Docker Development:**
- Docker (latest stable)
- Docker Compose (latest stable)

### Environment Variables

Create a `.env` file in the root directory (do not commit to git):

```bash
# Database
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/spire_board
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=spire_board

# Backend
BACKEND_HOST=0.0.0.0
BACKEND_PORT=8080

# Frontend
FRONTEND_PORT=3000
API_BASE_URL=http://localhost:8080
```

### Docker Setup

**Start development environment:**
```bash
docker-compose up -d
```

**Stop development environment:**
```bash
docker-compose down
```

**View logs:**
```bash
docker-compose logs -f
```

### IDE/Editor Recommendations

**For Rust:**
- VS Code with rust-analyzer extension
- IntelliJ IDEA with Rust plugin

**For PureScript:**
- VS Code with PureScript IDE extension
- Emacs with psc-ide mode
- Vim with purescript-vim

## Troubleshooting

### Common Issues

**Document common problems and solutions as they arise:**

| Issue | Solution |
|-------|----------|
| TBD   | TBD      |

### Getting Help

- Check existing documentation
- Search codebase for similar implementations
- Review git history for context
- Ask specific questions with context

## Performance Considerations

**Document as performance requirements emerge:**
- Critical performance paths
- Optimization strategies
- Profiling approaches
- Known bottlenecks

## Security Guidelines

1. **Input Validation**
   - Validate at system boundaries
   - Sanitize user input
   - Use allowlists over denylists

2. **Authentication & Authorization**
   - Follow established auth patterns
   - Never hardcode credentials
   - Use environment variables for secrets

3. **Data Protection**
   - Encrypt sensitive data
   - Use secure communication protocols
   - Follow principle of least privilege

## Maintenance

### Updating This Document

This CLAUDE.md should be updated when:
- Project structure changes significantly
- New conventions are established
- Technologies are added or changed
- Common issues are discovered
- Workflows are modified

### Review Schedule

- Review quarterly or after major changes
- Keep information current and accurate
- Remove outdated information
- Add newly discovered patterns and practices

## Quick Reference

### Essential Commands

```bash
# Git
git status              # Check repository status
git diff                # Review changes
git log --oneline -10   # Recent commits

# Backend (Rust)
cd backend
cargo build             # Build backend
cargo run               # Run backend server
cargo test              # Run tests
cargo clippy            # Run linter
cargo fmt               # Format code

# Frontend (PureScript)
cd frontend
spago build             # Build frontend
spago build --watch     # Build with auto-reload
spago test              # Run tests
spago repl              # Start REPL

# Docker
docker-compose up -d    # Start all services
docker-compose down     # Stop all services
docker-compose logs -f  # View logs
docker-compose ps       # List running containers

# Database
docker-compose exec postgres psql -U postgres -d spire_board
```

### File Locations

- **Backend Source:** `backend/src/`
- **Backend Tests:** `backend/tests/`
- **Frontend Source:** `frontend/src/`
- **Frontend Tests:** `frontend/test/`
- **Database Migrations:** `backend/migrations/`
- **Docker Config:** `docker-compose.yml`
- **Documentation:** `docs/`
- **Build Output (Backend):** `backend/target/`
- **Build Output (Frontend):** `frontend/output/`

### Useful Patterns

**Document reusable code patterns as they emerge:**
- Error handling
- API calls
- State updates
- Component structure

---

## Document Metadata

- **Created:** 2026-01-02
- **Last Updated:** 2026-01-02
- **Version:** 1.1.0
- **Maintained By:** AI assistants working on spire-board
- **Status:** Technology stack defined, ready for implementation

---

**Note to AI Assistants:** This is a living document. As you work on the codebase and discover patterns, conventions, or important information, update this file to help future assistants. Keep it concise, accurate, and focused on practical guidance.
