# Gamers CM

Full-stack app for managing gaming communities: players, teams, matches and rankings.

- **API** (repository root): Rust, Axum, SQLx, PostgreSQL, JWT
- **Web** ([`frontend/web`](frontend/web)): Next.js, React, TypeScript, Tailwind CSS, Radix UI

## Features

- Sign up and log in with JWT authentication
- Create communities and manage their players
- Build teams from community players
- Record matches (parties) between teams and their winners
- Community stats and rankings
- Paginated list endpoints and Swagger docs at `/api-docs/`

## Running locally

Requirements: Rust, Node.js 18+, Docker and `sqlx-cli` (`cargo install sqlx-cli --no-default-features --features postgres`).

**1. Database**

```bash
docker compose up -d
```

**2. API** (from the repository root)

Create a `.env` file:

```env
DATABASE_URL=postgres://gamer_user:gamer_password@localhost:5432/gamer_cm
PORT=3001
JWT_SECRET=change-me
```

```bash
sqlx migrate run
cargo run
```

The API runs at `http://localhost:3001`, with docs at `http://localhost:3001/api-docs/`.

**3. Web**

```bash
cd frontend/web
cp .env.example .env.local
npm install
npm run dev
```

Open `http://localhost:3000`.

## Tests

```bash
cargo test                      # API
cd frontend/web && npm test     # Web
```

## Project structure

```
gamer-cm/
├── src/              # API (Clean Architecture)
│   ├── domain/       # Entities and repository traits
│   ├── application/  # Use cases
│   ├── infra/        # Postgres repositories and configs
│   └── presentation/ # Routes, DTOs and auth middleware
├── migrations/       # SQL migrations
└── frontend/
    └── web/          # Next.js app
```
