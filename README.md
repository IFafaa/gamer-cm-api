# Gamers CM - Player Community Management API

A comprehensive REST API built with Rust for managing gaming communities, players, teams, and events. Designed for scalability and performance in modern gaming environments.

## 🚀 Technologies Used

- **Rust** - Systems programming language for performance and safety
- **Axum** - Modern async web framework
- **SQLx** - Async SQL toolkit with compile-time query verification
- **PostgreSQL** - Robust relational database
- **Docker** - Containerization for local development

## ✨ Features

- **Community Management** - Create and administer gaming communities
- **Player Profiles** - User registration and comprehensive profile management
- **Team System** - Team formation, membership, and role management
- **Event Management** - Schedule tournaments, matches, and community events
- **RESTful Architecture** - Clean, documented API endpoints
- **Authentication & Authorization** - Secure user access control

## 📋 Prerequisites

Before you begin, make sure you have installed:

- **Rust** (latest stable version)
- **Docker** and **Docker Compose**
- **Git**

### Rust Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Docker Installation

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install docker.io docker-compose

# Add user to docker group (necessary to use without sudo)
sudo usermod -aG docker $USER
# ⚠️ IMPORTANT: Logout and login again after this command
```

## 🛠️ Setup and Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd gamer-cm
```

### 2. Configure the Database

```bash
# Start PostgreSQL via Docker
docker-compose up -d

# Check if container is running
docker ps
```

### 3. Configure Environment Variables

The `.env` file is already configured with the following variables:

```env
# PostgreSQL Database Configuration
DATABASE_URL=postgres://user:password@localhost:5432/postgres

# Server Configuration
PORT=8080
HOST=0.0.0.0

# Environment
ENVIRONMENT=development
```

### 4. Install and Run Migrations

```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Run database migrations
sqlx migrate run
```

### 5. Build and Run the Project

```bash
# Build the project
cargo build

# Run the server
cargo run
```

## 🚀 Running the Project

### Development

```bash
# Terminal 1: Start the database
docker-compose up -d

# Terminal 2: Run the server
cargo run
```

The server will be available at: `http://localhost:8080`

### Success Logs

When everything is working correctly, you will see:

```
🚀 Server started successfully!
🌐 Listening on http://0.0.0.0:8080
```

## 📁 Project Structure

```
gamer-cm/
├── src/
│   ├── application/          # Use cases and interfaces
│   ├── domain/              # Domain entities
│   ├── infra/               # Infrastructure implementations
│   ├── presentation/        # Controllers and DTOs
│   └── shared/              # Shared utilities
├── migrations/              # Database migrations
├── docker-compose.yml       # PostgreSQL configuration
├── .env                     # Environment variables
└── Cargo.toml              # Rust dependencies
```

## 🧪 Testing the API

```bash
# Test if the server is running
curl http://localhost:8080

# Or use any HTTP client like Postman, Insomnia, etc.
```