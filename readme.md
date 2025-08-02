# Prod Ready HTTP Server Example

A production-ready HTTP server built with Axum, following enterprise-level architectural patterns and best practices.

## 🏗️ Architecture

This project follows a clean, layered architecture with clear separation of concerns:

```
src/
├── main.rs                      # Application entry point
├── config.rs                    # Configuration management
├── domain/                      # Domain models and business logic
│   ├── entities.rs              # Domain entities
│   ├── errors.rs                # Error types and handling
│   ├── requests.rs              # Request DTOs with validation
│   └── responses.rs             # Response DTOs
├── repositories/                # Data access layer
│   └── user.rs                  # User repository implementation
├── services/                    # Business logic layer
│   └── user.rs                  # User service with business rules
├── handlers/                    # HTTP request handlers
│   ├── health.rs                # Health check endpoints
│   └── user.rs                  # User CRUD endpoints
├── routes/                      # Route definitions
│   ├── health.rs                # Health route configuration
│   └── user.rs                  # User route configuration
├── middleware/                  # Custom middleware
│   └── logging_middleware.rs    # Request ID tracking
└── utils/                       # Shared utilities
```

## 🚀 Features

- **Clean Architecture**: Domain-driven design with clear layer separation
- **Error Handling**: Comprehensive error types with proper HTTP status mapping
- **Validation**: Input validation at multiple layers
- **Logging**: Structured logging with tracing
- **Configuration**: Environment-based configuration management
- **Middleware**: Request ID tracking, CORS, and HTTP tracing
- **Type Safety**: Full type safety with proper error propagation
- **Enterprise Ready**: Production-ready patterns and practices

## 📋 API Endpoints

### Health
- `GET /` - Welcome message
- `GET /api/v1/health` - Health check

### Users
- `GET /api/v1/users` - List users (with pagination)
- `POST /api/v1/users` - Create user
- `GET /api/v1/users/:id` - Get user by ID
- `PUT /api/v1/users/:id` - Update user
- `DELETE /api/v1/users/:id` - Delete user
- `GET /api/v1/users/:id/profile` - Get user profile

## 🛠️ Setup and Installation

### Prerequisites
- Rust 1.70+ 
- Cargo

### Installation

1. **Clone and setup:**
```bash
git clone <repository>
```

2. **Configure environment:**
```bash
cp .env.example .env
# Edit .env with your preferred settings
```

3. **Build and run:**
```bash
cargo build
cargo run
```

The server will start on `http://127.0.0.1:3000` by default.

## 🔧 Configuration

Configuration is managed through environment variables and config files:

### Environment Variables
All configuration can be set via environment variables with the `APP_` prefix:

```bash
APP_ENVIRONMENT=production
APP_SERVER__HOST=0.0.0.0
APP_SERVER__PORT=8080
APP_LOGGING__LEVEL=warn
```

### Config Files (Optional)
- `config/default.toml` - Default configuration
- `config/development.toml` - Development overrides
- `config/production.toml` - Production overrides
- `config/local.toml` - Local overrides (gitignored)

## 🧪 Testing

### Manual Testing with curl

```bash
# Health check
curl http://127.0.0.1:3000/api/v1/health

# List users
curl http://127.0.0.1:3000/api/v1/users

# Create user
curl -X POST http://127.0.0.1:3000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com","age":30}'

# Get user (replace {id} with actual UUID)
curl http://127.0.0.1:3000/api/v1/users/{id}

# Update user
curl -X PUT http://127.0.0.1:3000/api/v1/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"name":"John Smith"}'

# Delete user
curl -X DELETE http://127.0.0.1:3000/api/v1/users/{id}
```

### Run Tests
```bash
cargo test
```

## 📊 Logging

The application uses structured logging with different levels:

```bash
# Debug mode
RUST_LOG=debug cargo run

# Trace mode (very verbose)
RUST_LOG=trace cargo run

# Production mode (errors and warnings only)
RUST_LOG=warn cargo run
```

## 🔒 Error Handling

The application implements comprehensive error handling:

- **Domain Errors**: Business logic errors (user not found, validation failures)
- **HTTP Mapping**: Proper HTTP status codes for different error types
- **Structured Responses**: Consistent error response format
- **Request Tracking**: Each request gets a unique ID for debugging

## 🏢 Production Considerations

This codebase is designed with production deployment in mind:

### Scalability
- Stateless design for horizontal scaling
- Repository pattern for easy database integration
- Service layer for business logic isolation

### Monitoring
- Health check endpoints
- Request ID tracking
- Structured logging for observability
- Configurable log levels

### Security
- Input validation at multiple layers
- Type-safe request/response handling
- CORS configuration
- Ready for authentication middleware

### Configuration
- Environment-based configuration
- Separate configs for different environments
- Secure defaults

## 🔄 Development Workflow

```bash
# Development with auto-reload
cargo install cargo-watch
cargo watch -x run

# Format code
cargo fmt

# Lint code
cargo clippy

# Check for issues
cargo check
```

## 📚 Dependencies

Key dependencies and their purposes:

- **axum**: Modern web framework
- **tokio**: Async runtime
- **tower/tower-http**: Middleware and HTTP utilities
- **serde**: Serialization/deserialization
- **uuid**: Unique identifiers
- **tracing**: Structured logging
- **anyhow/thiserror**: Error handling
- **config**: Configuration management

## 🚀 Next Steps

Ready for extension with:

- Database integration (PostgreSQL, SQLite)
- Authentication & authorization (JWT, OAuth)
- API documentation (OpenAPI/Swagger)
- Metrics and monitoring (Prometheus)
- Caching (Redis)
- Message queues
- Docker containerization
- CI/CD pipeline integration