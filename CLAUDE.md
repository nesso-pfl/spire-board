# CLAUDE.md - AI Assistant Quick Reference

**This is a quick reference guide for AI assistants. For detailed documentation, see [docs/](./docs/).**

## Project Overview

**Name:** spire-board
**Repository:** nesso-pfl/spire-board
**Status:** Initial setup phase

### Description

Web application with Rust backend and PureScript frontend:
- **Backend:** REST API + WebSocket (Actix Web + SQLx + PostgreSQL)
- **Frontend:** SPA (PureScript + Halogen)
- **Monorepo:** Turborepo for build orchestration

## Directory Structure

```
spire-board/
├── backend/              # Rust API server
│   ├── src/              # Rust source code
│   ├── migrations/       # SQLx database migrations
│   ├── Cargo.toml        # Rust dependencies
│   └── package.json      # Turborepo scripts
├── frontend/             # PureScript web client
│   ├── src/              # PureScript source code
│   ├── spago.yaml        # Spago configuration
│   └── package.json      # Turborepo scripts
├── docs/                 # Detailed documentation
│   ├── SETUP.md          # Setup instructions
│   ├── DEVELOPMENT.md    # Development workflows
│   ├── CONVENTIONS.md    # Coding standards
│   └── ARCHITECTURE.md   # Architecture guide
├── compose.yaml          # Docker services (PostgreSQL)
├── turbo.json            # Turborepo configuration
├── package.json          # Root workspace config
└── .env.example          # Environment variables template
```

## Technology Stack

### Backend
- **Language:** Rust (edition 2021)
- **Web:** Actix Web 4.9 + actix-ws
- **Database:** PostgreSQL 17 + SQLx 0.8
- **Async:** Tokio 1.42

### Frontend
- **Language:** PureScript
- **UI:** Halogen (declarative framework)
- **Build:** Spago
- **Reference:** [Real World Halogen](https://thomashoneyman.com/guides/real-world-halogen/)

### Infrastructure
- **Monorepo:** Turborepo
- **Container:** Docker Compose
- **Node:** >=20.0.0, npm >=10.0.0

## Critical Git Rules

⚠️ **NEVER push directly to main branch**
⚠️ **Branch naming:** `claude/feature-name-{sessionId}` (must start with `claude/` and end with session ID)
⚠️ **Retry on network failure:** Up to 4 times with exponential backoff (2s, 4s, 8s, 16s)

### Commit Message Format

```
<type>: <short description>

<optional details>
```

**Types:** `feat`, `fix`, `refactor`, `docs`, `test`, `chore`

**Examples:**
- `feat: add user authentication`
- `fix: resolve database connection leak`
- `docs: update API documentation`

## Essential Commands

### Quick Start
```bash
npm install                    # Install dependencies
cp .env.example .env          # Create environment file
docker compose up -d          # Start PostgreSQL
npm run dev                   # Run all dev servers
```

### Turborepo (Recommended)
```bash
npm run dev         # Run all development servers
npm run build       # Build all workspaces
npm run test        # Run all tests
npm run lint        # Lint all code
npm run format      # Format all code
```

### Backend (from backend/)
```bash
cargo watch -x run          # Dev server with hot reload
cargo build --release       # Production build
cargo test                  # Run tests
cargo clippy                # Lint
cargo fmt                   # Format
cargo sqlx migrate run      # Run migrations
```

### Frontend (from frontend/)
```bash
spago build --watch         # Dev build with hot reload
spago build                 # Production build
spago test                  # Run tests
spago repl                  # Start REPL
```

### Docker
```bash
docker compose up -d        # Start services
docker compose down         # Stop services
docker compose logs -f      # View logs
```

## Key Conventions

### Rust
- Files: `snake_case.rs`
- Functions: `snake_case`
- Types: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- **Error handling:** Use `Result<T, E>` + `thiserror`
- **Testing:** `#[cfg(test)] mod tests`

### PureScript
- Files: `PascalCase.purs`
- Functions: `camelCase`
- Types: `PascalCase`
- Modules: `PascalCase.Nested.Module`
- **Error handling:** `Either`, `Maybe`, `Aff`
- **Architecture:** Capability pattern + AppM monad

### Security
- ✅ Parameterized queries (SQLx prevents SQL injection)
- ✅ Input validation at boundaries
- ✅ Environment variables for secrets (never hardcode)
- ❌ Never commit `.env` file

## Development Guidelines

### Before Making Changes
1. **Read files first** - Never modify unread code
2. **Research existing patterns** - Use Explore agent for codebase exploration
3. **Follow existing style** - Match surrounding code

### When Making Changes
1. **Prefer editing over creating** - Only create files when necessary
2. **Minimize scope** - Only change what's needed
3. **No over-engineering** - Keep solutions simple
4. **Remove unused code** - Delete, don't comment out

### Code Review Checklist
- [ ] All tests pass
- [ ] Code follows conventions
- [ ] Security vulnerabilities addressed
- [ ] No unrelated changes
- [ ] Documentation updated (if needed)

## File Locations

- **Backend source:** `backend/src/`
- **Backend tests:** `backend/tests/`
- **Backend migrations:** `backend/migrations/`
- **Frontend source:** `frontend/src/`
- **Frontend tests:** `frontend/test/`
- **Documentation:** `docs/`
- **Docker config:** `compose.yaml`
- **Build output (Backend):** `backend/target/` (gitignored)
- **Build output (Frontend):** `frontend/output/` (gitignored)

## Environment Variables

See `.env.example` for template. Key variables:

```bash
# Database
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/spire_board

# Backend
BACKEND_HOST=0.0.0.0
BACKEND_PORT=8080
RUST_LOG=info

# Frontend
FRONTEND_PORT=3000
API_BASE_URL=http://localhost:8080
```

**Note:** `NODE_ENV` is set automatically by npm scripts - do not add to `.env`

## Quick Troubleshooting

| Issue | Solution |
|-------|----------|
| Port in use | `lsof -i :8080` or `lsof -i :5432`, then kill process |
| Postgres connection fails | Check `docker compose ps` and verify DATABASE_URL |
| Rust build fails | `cargo clean && cargo build` |
| PureScript build fails | `rm -rf output .spago && spago build` |

## Documentation Index

For detailed information, see:

- **[docs/SETUP.md](./docs/SETUP.md)** - Installation and setup
- **[docs/DEVELOPMENT.md](./docs/DEVELOPMENT.md)** - Git workflow, commands, debugging
- **[docs/CONVENTIONS.md](./docs/CONVENTIONS.md)** - Coding standards and best practices
- **[docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)** - System design and patterns

## References

- [Real World Halogen Guide](https://thomashoneyman.com/guides/real-world-halogen/) - Frontend architecture
- [Real World Halogen Example](https://github.com/thomashoneyman/purescript-halogen-realworld) - Reference code
- [Actix Web](https://actix.rs/) - Backend framework
- [SQLx](https://github.com/launchbadge/sqlx) - Database toolkit
- [Turborepo](https://turbo.build/) - Monorepo tool

---

**Version:** 2.0.0
**Last Updated:** 2026-01-02
**Status:** Technology stack defined, ready for implementation

---

**Note to AI Assistants:** This is a quick reference. For detailed instructions, always consult the docs/ directory. Keep this file concise to minimize token usage.
