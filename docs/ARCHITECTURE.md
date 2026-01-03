# Architecture Guide

This document describes the architecture, design patterns, and technical decisions for spire-board.

## Overall Architecture

### Client-Server Separation

spire-board follows a strict client-server architecture:

- **Backend and frontend are completely separate applications**
- **Communication via REST API and WebSocket**
- **Backend serves as API-only server** (no server-side rendering)
- **Frontend is a single-page application** (SPA)

### Technology Stack

**Backend:**
- Language: Rust (latest stable)
- Web Framework: Actix Web
- Database: PostgreSQL 17
- ORM: SQLx (compile-time verified queries)
- Async Runtime: Tokio

**Frontend:**
- Language: PureScript
- UI Framework: Halogen
- Build Tool: Spago
- Package Manager: Spago

**Infrastructure:**
- Containerization: Docker & Docker Compose
- Monorepo: Turborepo
- Database: PostgreSQL 17

## Backend Architecture

### Design Patterns

#### RESTful API Design

- **Resource-oriented URLs:** `/api/users/{id}`
- **HTTP methods:** GET, POST, PUT, PATCH, DELETE
- **Status codes:** Proper use of 2xx, 4xx, 5xx
- **JSON responses:** Consistent response format

**Example:**
```rust
// GET /api/users/{id}
async fn get_user(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let user = user_service::find_by_id(&path).await?;
    Ok(HttpResponse::Ok().json(user))
}
```

#### WebSocket for Real-Time Communication

- **Persistent connections** for live updates
- **Message-based protocol**
- **Actix actors** for connection management

#### Actix Web Actors for Concurrency

- **Actor model** for concurrent operations
- **Message passing** between actors
- **Supervised execution** for fault tolerance

### Database Layer

#### SQLx for Type-Safe Queries

- **Compile-time query verification**
- **Type-safe result mapping**
- **Async/await support**

**Example:**
```rust
let user = sqlx::query_as!(
    User,
    "SELECT id, name, email FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await?;
```

#### Migration Strategy

- **Version-controlled migrations** in `backend/migrations/`
- **Reversible migrations** (`.up.sql` / `.down.sql` format)
- **Use SQLx CLI:** `cargo sqlx migrate add -r <name>`
- **Rollback support:** `cargo sqlx migrate revert`

### Error Handling

#### Custom Error Types

Use `thiserror` for domain errors:

```rust
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}
```

#### Error Response Format

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "User not found",
    "details": {}
  }
}
```

### Logging and Observability

- **Structured logging** with `tracing`
- **Log levels:** ERROR, WARN, INFO, DEBUG, TRACE
- **Environment-based:** `RUST_LOG=info`

## Frontend Architecture

### Design Patterns

#### Component-Based Architecture (Halogen)

- **Declarative UI** with type-safe components
- **Unidirectional data flow**
- **Parent-child component communication**

#### Capability Pattern for Dependency Injection

Following [Real World Halogen](https://thomashoneyman.com/guides/real-world-halogen/):

```purescript
class Monad m <= ManageUser m where
  getUser :: UserId -> m (Maybe User)
  createUser :: UserInput -> m User
  updateUser :: UserId -> UserInput -> m User
```

Benefits:
- **Testable:** Mock capabilities in tests
- **Flexible:** Swap implementations
- **Type-safe:** Compiler-verified contracts

#### AppM Monad for Application-Wide Effects

```purescript
newtype AppM a = AppM (ReaderT Env Aff a)

type Env =
  { apiUrl :: String
  , logLevel :: LogLevel
  }
```

Benefits:
- **Global configuration** accessible everywhere
- **Effect management** in one place
- **Easy to test** with different environments

### State Management

#### Component-Level State

```purescript
type State =
  { users :: Array User
  , loading :: Boolean
  , error :: Maybe String
  }
```

#### Application-Level State

- **Global state** via AppM environment
- **URL-based state** via routing
- **No global state library needed** (leverage type system)

### Type-Safe API Client

```purescript
module Api.User where

import Prelude
import Data.Either (Either)
import Effect.Aff (Aff)

type ApiError = String

getUser :: UserId -> Aff (Either ApiError User)
getUser id = do
  response <- get ("/api/users/" <> id)
  pure $ decodeUser response
```

### Routing

- **Hash-based routing** for SPA
- **Type-safe routes** with sum types
- **Route parsing** with purescript-routing

```purescript
data Route
  = Home
  | UserProfile UserId
  | Settings
```

## Data Flow

### Request Flow

1. **Frontend** makes HTTP/WebSocket request to backend
2. **Backend** validates request
3. **Backend** processes request, queries database via SQLx
4. **Backend** returns JSON response
5. **Frontend** updates UI based on response

### WebSocket Flow

1. **Client** establishes WebSocket connection
2. **Server** spawns actor to manage connection
3. **Bi-directional** message exchange
4. **Server** pushes updates to clients
5. **Client** updates UI reactively

## Security Architecture

### Authentication

- **Token-based authentication** (JWT or similar)
- **Secure token storage** (HttpOnly cookies)
- **Token refresh** mechanism

### Authorization

- **Role-based access control** (RBAC)
- **Permission checks** at API layer
- **Resource ownership** validation

### Data Protection

- **HTTPS only** in production
- **Password hashing** with bcrypt/argon2
- **SQL injection prevention** via parameterized queries (SQLx)
- **XSS prevention** via proper escaping (Halogen handles this)
- **CORS configuration** for API

## Performance Considerations

### Backend

- **Connection pooling** for database
- **Async I/O** with Tokio
- **Lazy evaluation** where appropriate
- **Caching** for expensive operations

### Frontend

- **Code splitting** for large applications
- **Lazy loading** of components
- **Minimal re-renders** (Halogen's virtual DOM)
- **Debouncing** user input

### Database

- **Indexes** on frequently queried columns
- **Query optimization** via EXPLAIN
- **Connection limits** to prevent exhaustion

## Scalability

### Horizontal Scaling

- **Stateless backend** for easy horizontal scaling
- **Load balancer** for traffic distribution
- **Session storage** in database or Redis

### Database Scaling

- **Read replicas** for read-heavy workloads
- **Connection pooling** to manage connections
- **Partitioning** for large tables

## Testing Strategy

### Backend Testing

**Unit Tests:**
- Test individual functions
- Mock database with test fixtures

**Integration Tests:**
- Test API endpoints end-to-end
- Use test database

**Example:**
```rust
#[actix_web::test]
async fn test_get_user() {
    let app = test::init_service(App::new().route("/users/{id}", web::get().to(get_user))).await;
    let req = test::TestRequest::get().uri("/users/123").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}
```

### Frontend Testing

**Unit Tests:**
- Test pure functions
- Test component logic

**Component Tests:**
- Test component rendering
- Test user interactions

## Deployment Architecture

### Development

- **Docker Compose** for local development
- **Hot reload** for rapid iteration
- **Local PostgreSQL** in container

### Production (Future)

- **Containerized deployment** (Docker)
- **Orchestration** (Kubernetes or similar)
- **Managed database** (AWS RDS, Azure Database, etc.)
- **CDN** for frontend assets
- **CI/CD pipeline** for automated deployment

## Module Organization

### Backend Structure

```
backend/src/
├── main.rs              # Entry point
├── config/              # Configuration
├── handlers/            # HTTP handlers
├── models/              # Data models
├── services/            # Business logic
├── db/                  # Database utilities
├── middleware/          # Actix middleware
└── error.rs             # Error types
```

### Frontend Structure

```
frontend/src/
├── Main.purs            # Entry point
├── Component/           # UI components
├── Data/                # Data types
├── Api/                 # API clients
├── Capability/          # Capability interfaces
├── Page/                # Page components
└── AppM.purs            # Application monad
```

## Design Decisions

### Why Rust for Backend?

- **Performance:** Comparable to C/C++
- **Safety:** Memory safety without GC
- **Concurrency:** Fearless concurrency with ownership
- **Ecosystem:** Mature web frameworks (Actix, Axum)

### Why PureScript for Frontend?

- **Type Safety:** Strong static typing catches errors
- **Functional:** Pure functions, immutable data
- **No Runtime Errors:** Well-typed programs don't crash
- **Halogen:** Robust UI framework

### Why SQLx over Diesel?

- **Async Support:** First-class async/await
- **Compile-Time Checks:** Query verification at compile time
- **Simplicity:** Less complex than Diesel
- **Performance:** Direct SQL with type safety

### Why Turborepo?

- **Monorepo Management:** Efficient multi-package builds
- **Caching:** Incremental builds
- **Task Pipeline:** Parallel task execution
- **Developer Experience:** Simple configuration

## Data Model Design

### Design Philosophy

The data model for spire-board follows these core principles:

1. **Separation of Concerns**: Clear distinction between master data, session data, and ephemeral data
2. **Flexibility over Normalization**: Use JSONB for complex, evolving data structures
3. **Type Safety at Application Layer**: Leverage Rust's type system for validation
4. **Future-Proof Schema**: Design for extensibility without frequent migrations

### Data Persistence Levels

Game data is categorized into three persistence levels:

#### 1. Master Data (Permanent Storage - PostgreSQL)

Static game definitions that rarely change:
- **Card definitions** (player cards, event cards, item cards)
- **Enemy definitions** (stats, attack patterns)
- **Map node templates** (shop, combat, campfire, event types)

**Characteristics:**
- Stored in PostgreSQL with proper schema
- Versioned via migrations
- Shared across all game sessions

#### 2. Session Data (Conditional Persistence - PostgreSQL)

Player progress and game state that needs to be saved:
- **Game runs** (active playthroughs)
- **Player state** (HP, gold, current deck)
- **Map progress** (current node, visited nodes)

**Characteristics:**
- Persisted if save/load functionality is required
- Can be deleted when run is completed
- Player-specific data

#### 3. Ephemeral Data (In-Memory Only - Rust Structs)

Temporary state during gameplay that doesn't need persistence:
- **Combat state** (current turn, energy, hand, draw/discard piles)
- **Shop inventory** (available cards/relics for current visit)
- **Event choices** (active event options)

**Characteristics:**
- Managed in-memory via Rust structs
- Discarded when scene/encounter ends
- Not stored in database

### JSONB vs Normalization Trade-off

#### Decision: Use JSONB for Card Effects

**Rationale:**
Card effects are highly variable and complex. A normalized approach would require:
- Multiple tables (`card_effects`, `effect_parameters`, `effect_conditions`)
- Frequent schema migrations for new effect types
- Complex JOIN queries to reconstruct card data

**JSONB Advantages:**
- ✅ **Flexibility**: Add new effect types without migrations
- ✅ **Simplicity**: Single query to fetch complete card data
- ✅ **Developer Experience**: Easy to understand and modify
- ✅ **Performance**: GIN index for effect searches

**JSONB Disadvantages:**
- ❌ **Type Safety**: JSON structure not enforced by database
- ❌ **Query Complexity**: More difficult to query specific effect types
- ❌ **Schema Evolution**: Changes require application-level migration

**Mitigation Strategy:**
- Use Rust's strong type system to validate JSON structure
- Define clear JSON schemas in documentation
- Use serde for serialization/deserialization with strict types

**When to Use JSONB:**
- Complex, nested data structures
- Frequently changing schemas
- Data that varies significantly between entities
- Performance is acceptable (indexed JSONB is fast)

**When to Normalize:**
- Simple, stable data structures
- Frequent queries on specific fields
- Need for referential integrity
- Complex aggregations required

### Card Upgrade Strategy

**Approach: Separate Entities with Reference**

Each upgraded card is a separate database row, linked via `base_card_id`:

```sql
-- Base card
base_card_id: NULL

-- Upgraded card
base_card_id: <UUID of base card>
```

**Rationale:**
- ✅ **Simplicity**: Each card is self-contained
- ✅ **Flexibility**: Upgrades can change any property (cost, effects, etc.)
- ✅ **Query Efficiency**: Direct lookup without conditional logic
- ✅ **Extensibility**: Supports multiple upgrade paths in future

**Alternative Considered (Single Entity):**
```sql
-- Store both versions in one row
is_upgraded: BOOLEAN
base_cost: INTEGER
upgraded_cost: INTEGER
base_effects: JSONB
upgraded_effects: JSONB
```

**Rejected Because:**
- ❌ Violates single responsibility principle
- ❌ More complex queries
- ❌ Difficult to support multiple upgrade paths
- ❌ Wastes space if only base card exists

### Effect Data Structure

Effects are stored as JSONB arrays with a consistent interface:

```json
[
  {
    "type": "block",
    "value": 3,
    "target": "any_player"
  },
  {
    "type": "add_to_deck",
    "card": "dizziness",
    "position": "top",
    "target": "self",
    "metadata": {}
  }
]
```

**Common Fields:**
- `type` (string, required): Effect identifier
- `target` (string, required): Target specification
- `value` (integer, optional): Numeric parameter
- `metadata` (object, optional): Effect-specific data

**Design Benefits:**
- Consistent interface across all effect types
- Easy to add new effect types
- Self-documenting (type field indicates behavior)
- Rust can deserialize to enum variants

**Future Considerations:**
- Conditional effects: `{"type": "conditional", "condition": {...}, "then": {...}}`
- Dynamic values: `{"type": "damage", "value_source": "hand_size", "multiplier": 2}`
- Effect chains: `{"type": "sequence", "effects": [...]}`

### Schema Versioning Strategy

**Migrations:**
- Use reversible migrations (`.up.sql` / `.down.sql`)
- Each migration is atomic and testable
- Document breaking changes in migration comments
- Keep migrations small and focused

**JSON Schema Evolution:**
- Application-level validation for JSONB fields
- Version field in JSON for future compatibility
- Gradual migration via application code
- Document JSON schema changes in `DATABASE.md`

### Database References

For detailed schema information, see:
- [Database Schema Documentation](./DATABASE.md) - Complete table definitions and ER diagrams
- [API Specification](./openapi.yaml) - API contracts and data formats

## Future Considerations

As the project grows, consider:

- **GraphQL** for flexible API queries
- **Event sourcing** for audit trails
- **CQRS** for read/write separation
- **Microservices** if domains become complex
- **Service mesh** for inter-service communication

## References

- [Real World Halogen Guide](https://thomashoneyman.com/guides/real-world-halogen/) - Frontend architecture patterns
- [Real World Halogen Example](https://github.com/thomashoneyman/purescript-halogen-realworld) - Reference implementation
- [Actix Web Documentation](https://actix.rs/) - Backend framework
- [SQLx Documentation](https://github.com/launchbadge/sqlx) - Database toolkit
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - API design
