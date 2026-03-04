# CLAUDE.md - Gamer CM API

## Overview
API REST para gerenciamento de comunidades gamer. Organiza players, teams, parties (partidas) e rankings.

## Stack
- **Rust** (1.85+) + **Axum** (web framework)
- **SQLx** (compile-time checked queries) + **PostgreSQL 13**
- **JWT** (jsonwebtoken) para autenticação
- **Argon2** para hash de senhas
- **Swagger UI** em `/api-docs/`

## Arquitetura - Clean Architecture

```
src/
├── domain/           # Entidades + traits de repositório (interfaces)
│   ├── user.rs       # User entity + UserRepository trait
│   ├── community.rs  # Community entity + CommunityRepository trait
│   ├── player.rs     # Player entity + PlayerRepository trait
│   ├── team.rs       # Team entity + TeamRepository trait
│   └── party.rs      # Party entity + PartyRepository trait
├── application/
│   ├── use_cases/    # Lógica de negócio (1 arquivo por operação)
│   └── interfaces/   # DTOs de saída (result interfaces)
├── infra/
│   ├── configs/      # DB, CORS, env, listener, swagger
│   └── db/           # Implementações Postgres dos repository traits
├── presentation/
│   ├── routes/       # Handlers HTTP (community, player, team, party, stats, auth)
│   ├── dtos/         # DTOs de entrada (create, update, delete)
│   └── middleware/    # Auth middleware (JWT extraction → AuthenticatedUser)
└── shared/           # Utils: api_error, api_response, jwt_service, password_service, date_time, validate_dto
```

## Modelo de Domínio

```
User (1) ──→ (N) Community (1) ──→ (N) Player
                              (1) ──→ (N) Team
                              (1) ──→ (N) Party

Team (N) ←──→ (N) Player     (via team_players)
Party (N) ←──→ (N) Team      (via party_teams)
Party (0..1) ──→ Team         (team_winner_id)
```

- Todas as entidades têm soft delete (`enabled: bool`)
- Timestamps: `created_at`, `updated_at` (PrimitiveDateTime do crate `time`)

## Endpoints

### Auth (público)
- `POST /auth/register` → RegisterDto → AuthResponseDto (token + user)
- `POST /auth/login` → LoginDto → AuthResponseDto

### Communities (autenticado)
- `POST /communities` → CreateCommunityDto
- `GET /communities` → lista do user
- `GET /communities/{id}` → detalhes com players e teams
- `PUT /communities/{id}` → UpdateCommunityDto
- `DELETE /communities/{id}` → soft delete

### Players (autenticado)
- `POST /players` → CreatePlayerIntoCommunityDto
- `PUT /players/{id}` → UpdatePlayerDto
- `DELETE /players/{id}` → soft delete

### Teams (autenticado)
- `POST /teams` → CreateTeamIntoCommunityDto
- `PUT /teams/{id}` → UpdateTeamDto
- `POST /teams/add-players` → AddPlayersIntoTeamDto
- `PATCH /teams/delete-players` → DeletePlayersOfTeamDto

### Parties (autenticado)
- `POST /parties` → CreatePartyDto (min 2 teams)
- `GET /parties?community_id=X` → lista filtrada por community do user
- `GET /parties/{id}` → detalhes
- `PATCH /parties/end` → EndPartyDto (define winner)
- `DELETE /parties/{id}` → soft delete

### Stats (autenticado)
- `GET /stats/communities/{id}` → CommunityStats (rankings, win rates, top games)

## Patterns

### Ownership Validation
Todos os endpoints verificam que o recurso pertence ao user autenticado via `community_id → user_id`. Método chave: `CommunityRepository::belongs_to_user(community_id, user_id)`.

### Use Case Pattern
Cada operação é um struct com repositórios injetados via Arc<dyn Trait>. Execute recebe DTO + user_id, retorna `Result<T, (StatusCode, ApiErrorResponse)>`.

### Repository Pattern
Traits definidos em `domain/`, implementações Postgres em `infra/db/`. SQLx com compile-time query validation.

### Error Handling
- `ApiErrorResponse { message, timestamp }` em JSON
- Status codes semânticos: 400, 401, 403, 404, 409, 500

### Validation
- DTOs usam `validator` crate (derive Validate) ou validação manual
- `validate_dto()` helper em `shared/validate_dto.rs`

## Configuração

```bash
# .env
DATABASE_URL=postgres://user:password@localhost:5432/postgres
JWT_SECRET=your-secret-key
PORT=3000
```

## Comandos

```bash
# Dev
cargo run

# Build
cargo build --release

# Migrations
sqlx migrate run

# Preparar SQLx offline cache
cargo sqlx prepare -- --bin game_gc_rust

# Docker
docker compose up -d          # Postgres
DOCKER_BUILDKIT=1 docker build -t gamer-cm-api .
```

## Pendências conhecidas
- Sem testes (unitários ou integração)
- Dynamic query builder em `get_by_params` usa JsonValue (frágil) — preferir métodos dedicados
- Paginação: struct `PaginationMeta` existe mas não é usado
- Sem rate limiting
- Sem logging estruturado (tracing)
- JWT expira em 24h, sem refresh token
