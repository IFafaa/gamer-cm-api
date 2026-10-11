# CLAUDE.md - Gamer CM Front

## Overview
Frontend do Gamer CM — gerenciamento de comunidades gamer. Interface para criar/gerenciar communities, players, teams e parties (partidas).

## Stack
- **Next.js 15** (App Router + Turbopack)
- **React 19**
- **TypeScript**
- **Tailwind CSS**
- **Radix UI** (via shadcn/ui components)
- **Axios** para HTTP
- **Sonner** para toasts
- **Lucide** para ícones

## Arquitetura

```
src/
├── app/                          # Next.js App Router pages
│   ├── page.tsx                  # Root redirect
│   ├── layout.tsx                # Root layout (AuthProvider, Toaster)
│   ├── login/page.tsx            # Login form
│   ├── register/page.tsx         # Register form
│   ├── not-found.tsx             # 404
│   └── (authenticated)/          # Route group (requer auth)
│       ├── layout.tsx            # Layout com Header + Sidebar
│       ├── home/page.tsx         # Lista de communities
│       └── community/[id]/page.tsx  # Detalhes da community
├── components/
│   ├── ui/                       # shadcn/ui primitivos (button, card, dialog, input, etc.)
│   ├── Header.tsx                # Navbar com user info + logout
│   ├── Sidebar.tsx               # Lista de communities na lateral
│   ├── ProtectedRoute.tsx        # Guard de autenticação
│   ├── CommunityContent.tsx      # Página principal da community (players, teams, parties)
│   ├── CreateCommunityDialog.tsx
│   ├── CreatePlayerDialog.tsx
│   ├── CreateTeamDialog.tsx
│   ├── CreatePartyDialog.tsx
│   ├── AddPlayersToTeamDialog.tsx
│   ├── RemovePlayersFromTeamDialog.tsx
│   └── EndPartyDialog.tsx
├── contexts/
│   └── AuthContext.tsx            # JWT auth state (login, register, logout, token management)
├── services/                     # API calls (1 arquivo por entidade)
│   ├── auth.ts
│   ├── community.ts              # getCommunities, createCommunity, updateCommunity, deleteCommunity
│   ├── player.ts                 # createPlayer, updatePlayer, deletePlayer
│   ├── team.ts                   # createTeam, updateTeam, addPlayersToTeam, deletePlayersOfTeam
│   ├── party.ts                  # createParty, getParties(communityId?), endParty, deleteParty
│   └── stats.ts                  # getCommunityStats
├── types/
│   ├── auth.ts                   # User, AuthContextType, LoginRequest, RegisterRequest
│   └── community.ts              # Player, Team, Party, Community, CommunitiesResponse
└── lib/
    ├── api.ts                    # Axios instance com interceptors (auth token, 401 redirect)
    └── utils.ts                  # cn() helper (tailwind merge)
```

## Patterns

### Autenticação
- JWT em `localStorage` (`auth_token`, `auth_user`)
- `AuthContext` provê `login()`, `register()`, `logout()`, `isAuthenticated`
- Axios interceptor injeta `Authorization: Bearer <token>` em toda request
- 401 → limpa localStorage → redirect `/login`
- `ProtectedRoute` wraps authenticated pages

### API Communication
- Todas as calls em `services/` usam a instância Axios de `lib/api.ts`
- API base URL via `NEXT_PUBLIC_API_URL` (default: `http://localhost:3000`)
- Responses seguem `{ data: T, timestamp: string }` ou `{ data: T, meta?: PaginationMeta, timestamp }`
- Erros: `{ message: string, timestamp: string }`

### Component Pattern
- Dialogs usam Radix `AlertDialog` / `Dialog` controlados por state
- `CommunityContent` é o componente central — gerencia players, teams e parties de uma community
- Refresh pattern: `onCommunityUpdate` callback + `loadParties()` após mutações
- Parties filtradas server-side via `getParties(community.id)`

### Route Groups
- `(authenticated)/` group compartilha layout com Header + Sidebar
- `layout.tsx` do group usa `ProtectedRoute` para guardar acesso

## Configuração

```bash
# .env.local
NEXT_PUBLIC_API_URL=http://localhost:3000
```

## Comandos

```bash
npm install        # Instalar deps
npm run dev        # Dev server (Turbopack, porta 3001 se 3000 ocupada)
npx next build     # Build de produção
npm start          # Servir build
```

## UI Components (shadcn/ui)
Primitivos em `components/ui/`: button, card, dialog, alert-dialog, input, label, checkbox, radio-group, dropdown-menu, avatar. Todos Radix-based com Tailwind styling.

## Tipos principais

```typescript
interface Community {
  id: number; name: string; players: Player[]; teams: Team[];
  created_at: string; updated_at: string;
}

interface Player {
  id: number; nickname: string; community_id: number;
  created_at: string; updated_at: string;
}

interface Team {
  id: number; name: string; community_id: number; players: Player[];
  created_at: string; updated_at: string;
}

interface Party {
  id: number; community_id: number; game_name: string;
  team_winner_id?: number; finished_at?: string;
  teams: Team[]; created_at: string; updated_at: string;
}

interface CommunityStats {
  total_players: number; total_teams: number;
  total_parties: number; active_parties: number; finished_parties: number;
  team_rankings: TeamRanking[]; player_rankings: PlayerRanking[];
  most_played_games: GameStats[];
}
```

## Pendências conhecidas
- Sem testes (unit ou e2e)
- Stats/ranking ainda não tem UI (service pronto, falta componente)
- Update de community/player/team: services prontos, falta UI de edição inline
- Sem refresh token — logout forçado após 24h
- Sem i18n (interface em inglês, projeto brasileiro)
