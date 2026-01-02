# Project Setup Guide

This guide provides detailed instructions for setting up the spire-board development environment.

## Prerequisites

### Required Tools and Versions

**Backend Development:**
- Rust (latest stable) - Install via [rustup](https://rustup.rs/)
- Cargo (comes with Rust)
- PostgreSQL client tools (for local development)

**Frontend Development:**
- Node.js (LTS version, >=20.0.0) - Required for PureScript tooling
- npm (>=10.0.0)
- PureScript compiler - Install via npm: `npm install -g purescript`
- Spago - Install via npm: `npm install -g spago`

**Docker Development:**
- Docker (latest stable)
- Docker Compose (latest stable)

### IDE/Editor Recommendations

**For Rust:**
- VS Code with rust-analyzer extension
- IntelliJ IDEA with Rust plugin

**For PureScript:**
- VS Code with PureScript IDE extension
- Emacs with psc-ide mode
- Vim with purescript-vim

## Initial Setup

### 1. Clone Repository

```bash
git clone <repository-url>
cd spire-board
```

### 2. Install Dependencies

```bash
# Install Node.js dependencies (including Turborepo)
npm install
```

### 3. Environment Variables

Create a `.env` file from the example:

```bash
cp .env.example .env
```

Edit `.env` with your configuration:

```bash
# Database Configuration
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/spire_board
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=spire_board
POSTGRES_PORT=5432

# Backend Configuration
BACKEND_HOST=0.0.0.0
BACKEND_PORT=8080
RUST_LOG=info

# Frontend Configuration
FRONTEND_PORT=3000
API_BASE_URL=http://localhost:8080
```

**Important:** Never commit `.env` to version control. It's already in `.gitignore`.

### 4. Start Docker Services

Start PostgreSQL database:

```bash
docker compose up -d
```

Verify it's running:

```bash
docker compose ps
docker compose logs postgres
```

### 5. Backend Setup

```bash
cd backend

# Build backend
cargo build

# Run database migrations (when available)
cargo sqlx migrate run

# Start development server
cargo run
```

The backend should now be running at `http://localhost:8080`

Test it:
```bash
curl http://localhost:8080/health
```

### 6. Frontend Setup

```bash
cd frontend

# Install PureScript dependencies
spago build

# Start development server
spago build --watch
```

## Using Turborepo (Recommended)

Instead of running backend and frontend separately, use Turborepo:

```bash
# From project root

# Run all development servers
npm run dev

# Build all workspaces
npm run build

# Run all tests
npm run test
```

## Verification

After setup, verify everything works:

1. **Database**: `docker compose ps` shows postgres running
2. **Backend**: `curl http://localhost:8080/health` returns `{"status":"ok"}`
3. **Frontend**: Frontend build completes without errors

## Troubleshooting

### PostgreSQL Connection Issues

**Problem:** Backend can't connect to database

**Solution:**
1. Check PostgreSQL is running: `docker compose ps`
2. Check logs: `docker compose logs postgres`
3. Verify DATABASE_URL in `.env`
4. Ensure port 5432 is not in use by another process

### Rust Compilation Errors

**Problem:** `cargo build` fails

**Solution:**
1. Update Rust: `rustup update`
2. Clean build: `cargo clean && cargo build`
3. Check Cargo.toml for dependency conflicts

### PureScript Build Errors

**Problem:** `spago build` fails

**Solution:**
1. Clean PureScript cache: `rm -rf output .spago`
2. Rebuild: `spago build`
3. Check PureScript compiler version: `purs --version`

### Port Already in Use

**Problem:** Port 8080 or 5432 already in use

**Solution:**
1. Find process: `lsof -i :8080` or `lsof -i :5432`
2. Kill process or change port in `.env`

## Next Steps

After successful setup:
- Read [DEVELOPMENT.md](./DEVELOPMENT.md) for development workflows
- Read [CONVENTIONS.md](./CONVENTIONS.md) for coding standards
- Read [ARCHITECTURE.md](./ARCHITECTURE.md) for system design

## Clean Up

To stop and remove all services:

```bash
# Stop services
docker compose down

# Remove volumes (WARNING: deletes database data)
docker compose down -v
```
