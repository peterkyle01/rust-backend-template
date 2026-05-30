# Rust MVC Backend

> ⚠️ **Archived Repository**
>
> This repository is archived. Please use [peterkyle01/rust-neon-template](https://github.com/peterkyle01/rust-neon-template), which is now the preferred template.

A clean, production-ready Rust backend with JWT authentication and Neon PostgreSQL database.

## 🚀 Features

- **MVC Architecture**: Clean separation of concerns
- **JWT Authentication**: Secure token-based auth
- **Neon Database**: Serverless PostgreSQL
- **Password Security**: Bcrypt hashing
- **Input Validation**: Request validation
- **Docker Ready**: Simple containerization

## 📁 Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── database.rs          # Database connection
├── models.rs            # Data models
├── errors.rs            # Error handling
├── auth.rs              # Authentication logic
├── handlers/            # Controllers (MVC)
│   ├── auth.rs          # Auth endpoints
│   └── user.rs          # User endpoints
└── middleware/          # Middleware
    └── auth.rs          # Auth middleware
migrations/              # Database migrations
```

## 🏃‍♂️ Quick Start

### 1. Setup Neon Database

1. Go to [Neon Console](https://console.neon.tech)
2. Create a new project
3. Copy your connection string

### 2. Configure Environment

Create a `.env` file in the project root:

```bash
# Copy the example file
cp .env.example .env

# Edit with your actual values
DATABASE_URL=postgresql://username:password@your-hostname.neon.tech/database?sslmode=require
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRY_HOURS=24
BCRYPT_COST=12
HOST=0.0.0.0
PORT=8080
RUST_LOG=debug
```

### 3. Complete Setup

Run the automated setup process:

```bash
make setup
```

This will:

- Validate your `.env` configuration
- Install SQLx CLI if needed
- Run database migrations
- Build the project

### 4. Start Application

#### Development Mode

```bash
make run          # Run with cargo (hot reload with cargo-watch)
```

#### Production Mode

```bash
make release      # Build release binary
make run-prod     # Run release binary with .env configuration
```

The server will start at `http://localhost:8080`

## 🔗 API Endpoints

### Public Endpoints

- `GET /health` - Health check
- `POST /register` - User registration
- `POST /login` - User login

### Protected Endpoints

- `GET /me` - Get current user (requires Bearer token)

## 📖 API Usage

### Register User

```bash
curl -X POST http://localhost:8080/register
  -H "Content-Type: application/json"
  -d '{
    "email": "user@example.com",
    "password": "securepassword123",
    "first_name": "John",
    "last_name": "Doe"
  }'
```

### Login User

```bash
curl -X POST http://localhost:8080/login
  -H "Content-Type: application/json"
  -d '{
    "email": "user@example.com",
    "password": "securepassword123"
  }'
```

### Get User Info

```bash
curl -X GET http://localhost:8080/me
  -H "Authorization: Bearer YOUR_TOKEN"
```

## 🐳 Docker Deployment

### Build Image

```bash
make docker-build
```

### Run Container

```bash
make docker-run
```

Or manually:

```bash
docker build -t rust-backend .
docker run -p 8080:8080 --env-file .env rust-backend
```

## ⚙️ Configuration

Environment variables in `.env`:

| Variable           | Description            | Default                                 |
| ------------------ | ---------------------- | --------------------------------------- |
| `DATABASE_URL`     | Neon connection string | Required (PostgreSQL connection string) |
| `JWT_SECRET`       | JWT signing secret     | Required (recommended: 32+ characters)  |
| `JWT_EXPIRY_HOURS` | Token expiry time      | 24                                      |
| `BCRYPT_COST`      | Password hash cost     | 12                                      |
| `PORT`             | Server port            | 8080                                    |
| `HOST`             | Server host            | 0.0.0.0                                 |
| `RUST_LOG`         | Log level              | debug (development), info (production)  |

### Environment File Setup

1. **Copy the example file:**

   ```bash
   cp .env.example .env
   ```

2. **Update the `.env` file with your actual values:**

   - Replace `DATABASE_URL` with your Neon connection string
   - Set a strong `JWT_SECRET` (at least 32 characters)
   - Adjust other settings as needed

3. **Run the setup:**
   ```bash
   make setup
   ```

## 🛠️ Development

### Available Make Targets

Use `make help` to see all available targets:

```bash
# Setup and Development
make setup        # Complete project setup (recommended first run)
make build        # Build the project
make run          # Run the project locally (development mode)
make test         # Run tests
make clean        # Clean build artifacts

# Production
make release      # Build release binary
make run-prod     # Run release binary with .env configuration

# Database
make migrate      # Run database migrations

# Code Quality
make fmt          # Format code
make lint         # Run clippy linter
make check        # Run all checks (fmt + lint + test)

# Docker
make docker-build # Build Docker image
make docker-run   # Run Docker container
```

### Development Workflow

```bash
# Initial setup
make setup

# Development cycle
make fmt          # Format code
make lint         # Check for issues
make test         # Run tests
make run          # Start development server

# Before committing
make check        # Run all quality checks
```

## 📊 Database Schema

### Users Table

- `id` - UUID (Primary Key)
- `email` - String (Unique)
- `password_hash` - String
- `first_name` - String
- `last_name` - String
- `created_at` - Timestamp
- `updated_at` - Timestamp

## 🔒 Security

- Passwords hashed with bcrypt
- JWT tokens with expiration
- SQL injection protection
- Input validation
- HTTPS ready (SSL certificates)

## 📝 License

MIT License

A production-ready Rust backend with JWT authentication, PostgreSQL database, and Docker containerization.

## Features

- 🚀 **Fast & Secure**: Built with Axum web framework
- 🔐 **JWT Authentication**: Secure token-based authentication
- 🗄️ **PostgreSQL**: Robust database with migrations
- 🐳 **Docker**: Containerized for easy deployment
- 📝 **Validation**: Request validation with detailed error messages
- 🔒 **Password Hashing**: Bcrypt password hashing
- 📊 **Logging**: Structured logging with tracing
- 🏥 **Health Checks**: Built-in health check endpoint

## API Endpoints

### Public Endpoints

#### Health Check

```
GET /health
```

Returns server health status.

**Response:**

```json
{
  "status": "healthy",
  "timestamp": "2024-08-28T10:30:00Z"
}
```

#### Register User

```
POST /register
Content-Type: application/json
```

**Request Body:**

```json
{
  "email": "user@example.com",
  "password": "securepassword123",
  "first_name": "John",
  "last_name": "Doe"
}
```

**Response (201 Created):**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "created_at": "2024-08-28T10:30:00Z",
    "updated_at": "2024-08-28T10:30:00Z"
  }
}
```

#### Login User

```
POST /login
Content-Type: application/json
```

**Request Body:**

```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response (200 OK):**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "created_at": "2024-08-28T10:30:00Z",
    "updated_at": "2024-08-28T10:30:00Z"
  }
}
```

### Protected Endpoints

All protected endpoints require a Bearer token in the Authorization header:

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

#### Get Current User

```
GET /me
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "created_at": "2024-08-28T10:30:00Z",
  "updated_at": "2024-08-28T10:30:00Z"
}
```

## Error Responses

The API returns standardized error responses:

```json
{
  "error": "Error message description"
}
```

### HTTP Status Codes

- `200` - OK
- `201` - Created
- `400` - Bad Request (validation errors)
- `401` - Unauthorized (invalid/missing token)
- `403` - Forbidden
- `404` - Not Found
- `409` - Conflict (e.g., user already exists)
- `500` - Internal Server Error

## Quick Start

### Prerequisites

- Docker and Docker Compose
- Rust (if running locally)

### Using Docker Compose (Recommended)

1. **Clone and navigate to the project:**

   ```bash
   cd rust-backend
   ```

2. **Setup environment:**

   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Start the services:**

   ```bash
   make docker-build
   make docker-run
   ```

4. **Test the API:**
   ```bash
   curl http://localhost:8080/health
   ```

### Local Development (Recommended)

1. **Complete setup:**

   ```bash
   make setup
   ```

2. **Start the development server:**

   ```bash
   make run
   ```

3. **For production testing:**
   ```bash
   make release
   make run-prod
   ```

### Production Deployment

1. **Setup environment:**

   ```bash
   cp .env.example .env
   ```

2. **Update production variables in `.env`:**

   - Set your actual Neon `DATABASE_URL`
   - Set a secure `JWT_SECRET` (at least 32 characters)
   - Set `RUST_LOG=info` for production
   - Adjust other settings as needed

3. **Deploy:**
   ```bash
   make setup        # Complete setup and validation
   make release      # Build optimized binary
   make run-prod     # Run production server
   ```

### Local Development

1. **Install Rust and dependencies:**

   ```bash
   # Install Rust if not already installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Run complete setup
   make setup
   ```

2. **Start the development server:**
   ```bash
   make run
   ```

For manual setup:

1. **Install SQLx CLI:**

   ```bash
   cargo install sqlx-cli --no-default-features --features rustls,postgres
   ```

2. **Run migrations:**

   ```bash
   make migrate
   ```

3. **Build and run:**
   ```bash
   make build
   make run
   ```

## Configuration

Environment variables can be set in `.env` file or as system environment variables:

| Variable           | Description                    | Default                                                      |
| ------------------ | ------------------------------ | ------------------------------------------------------------ |
| `DATABASE_URL`     | PostgreSQL connection string   | `postgresql://postgres:password@localhost:5432/rust_backend` |
| `JWT_SECRET`       | Secret key for JWT signing     | Required                                                     |
| `JWT_EXPIRY_HOURS` | JWT token expiry time in hours | `24`                                                         |
| `BCRYPT_COST`      | Bcrypt hashing cost            | `12`                                                         |
| `PORT`             | Server port                    | `8080`                                                       |
| `HOST`             | Server host                    | `0.0.0.0`                                                    |
| `RUST_LOG`         | Log level                      | `info`                                                       |

## Database Schema

### Users Table

| Column          | Type         | Constraints                              |
| --------------- | ------------ | ---------------------------------------- |
| `id`            | UUID         | Primary Key, Default: uuid_generate_v4() |
| `email`         | VARCHAR(255) | Unique, Not Null                         |
| `password_hash` | VARCHAR(255) | Not Null                                 |
| `first_name`    | VARCHAR(100) | Not Null                                 |
| `last_name`     | VARCHAR(100) | Not Null                                 |
| `created_at`    | TIMESTAMPTZ  | Not Null, Default: NOW()                 |
| `updated_at`    | TIMESTAMPTZ  | Not Null, Default: NOW()                 |

## Security Features

- **Password Hashing**: Uses bcrypt with configurable cost
- **JWT Tokens**: Stateless authentication with expiration
- **Input Validation**: Request validation with detailed error messages
- **SQL Injection Protection**: Uses parameterized queries
- **CORS Support**: Configurable cross-origin requests
- **Security Headers**: Production-ready HTTP headers

## Monitoring & Observability

- **Health Check**: `/health` endpoint for load balancer health checks
- **Structured Logging**: JSON-formatted logs with tracing
- **Request Tracing**: Request/response logging with correlation IDs
- **Error Tracking**: Detailed error logging and reporting

## Testing the API

### Example requests using curl:

1. **Register a new user:**

   ```bash
   curl -X POST http://localhost:8080/register \
     -H "Content-Type: application/json" \
     -d '{
       "email": "test@example.com",
       "password": "securepassword123",
       "first_name": "Test",
       "last_name": "User"
     }'
   ```

2. **Login:**

   ```bash
   curl -X POST http://localhost:8080/login \
     -H "Content-Type: application/json" \
     -d '{
       "email": "test@example.com",
       "password": "securepassword123"
     }'
   ```

3. **Get current user info (replace TOKEN with actual token):**
   ```bash
   curl -X GET http://localhost:8080/me \
     -H "Authorization: Bearer TOKEN"
   ```

## License

This project is licensed under the MIT License.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request
