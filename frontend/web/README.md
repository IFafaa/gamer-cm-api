# Gamers CM - Web

Next.js frontend for [Gamers CM](../../README.md).

**Stack:** Next.js 15, React 19, TypeScript, Tailwind CSS, Radix UI (shadcn/ui), Axios, Jest.

## Running

The API must be running first (see the [root README](../../README.md)).

```bash
cp .env.example .env.local   # NEXT_PUBLIC_API_URL points to the API
npm install
npm run dev
```

Open `http://localhost:3000`.

## Scripts

| Command | Description |
| --- | --- |
| `npm run dev` | Dev server with Turbopack |
| `npm run build` | Production build |
| `npm run start` | Serve the production build |
| `npm run lint` | Lint |
| `npm test` | Run tests |

## Structure

```
src/
├── app/          # Pages (App Router); protected routes under (authenticated)/
├── components/   # UI components (ui/ holds the base Radix components)
├── contexts/     # React contexts (auth)
├── services/     # API calls
└── types/        # TypeScript types
```
