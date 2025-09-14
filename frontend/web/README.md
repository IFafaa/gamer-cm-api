# Gamers CM - Frontend

Uma aplicação web moderna construída com Next.js para gerenciamento de comunidades de jogadores. Interface intuitiva e responsiva para administrar comunidades, jogadores, times e eventos de gaming.

## 🚀 Tecnologias Utilizadas

- **Next.js 15** - Framework React para aplicações web modernas
- **React 19** - Biblioteca para interfaces de usuário
- **TypeScript** - Superset do JavaScript com tipagem estática
- **Tailwind CSS** - Framework CSS utilitário para design responsivo
- **Radix UI** - Componentes acessíveis e customizáveis
- **Axios** - Cliente HTTP para comunicação com a API
- **Lucide React** - Ícones modernos e consistentes

## ✨ Funcionalidades

- **Autenticação de Usuários** - Login e registro seguros
- **Gerenciamento de Comunidades** - Criação e administração de comunidades de gaming
- **Sistema de Jogadores** - Perfis de jogadores e gerenciamento de membros
- **Sistema de Times** - Formação e gerenciamento de times dentro das comunidades
- **Gerenciamento de Eventos** - Criação e administração de partidas e torneios
- **Interface Responsiva** - Design adaptável para desktop e mobile
- **Tema Escuro/Claro** - Suporte a modo escuro e claro
- **Componentes Reutilizáveis** - Biblioteca de componentes UI consistente

## 📋 Pré-requisitos

Antes de começar, certifique-se de ter instalado:

- **Node.js** (versão 18 ou superior)
- **npm** ou **yarn** ou **pnpm**
- **Git**

## 🛠️ Configuração e Instalação

### 1. Clone o Repositório

```bash
git clone <repository-url>
cd gamer-cm/gamer-cm-front
```

### 2. Instale as Dependências

```bash
# Usando npm
npm install

# Ou usando yarn
yarn install

# Ou usando pnpm
pnpm install
```

### 3. Configure as Variáveis de Ambiente

Crie um arquivo `.env.local` na raiz do projeto:

```env
# URL da API Backend
NEXT_PUBLIC_API_URL=http://localhost:8080

# Ambiente
NODE_ENV=development
```

### 4. Execute o Projeto

```bash
# Modo desenvolvimento
npm run dev

# Ou usando yarn
yarn dev

# Ou usando pnpm
pnpm dev
```

## 🚀 Executando o Projeto

### Desenvolvimento

```bash
# Terminal 1: Execute o backend (gamer-cm-api)
cd ../gamer-cm-api
cargo run

# Terminal 2: Execute o frontend
cd gamer-cm-front
npm run dev
```

A aplicação estará disponível em: `http://localhost:3000`

### Logs de Sucesso

Quando tudo estiver funcionando corretamente, você verá:

```
✓ Ready in 2.3s
✓ Local:        http://localhost:3000
✓ Network:      http://192.168.1.100:3000
```

## 📁 Estrutura do Projeto

```
gamer-cm-front/
├── src/
│   ├── app/                    # Páginas e layouts (App Router)
│   │   ├── (authenticated)/    # Rotas protegidas
│   │   ├── login/              # Página de login
│   │   └── register/           # Página de registro
│   ├── components/             # Componentes reutilizáveis
│   │   ├── ui/                 # Componentes base (Radix UI)
│   │   └── *.tsx               # Componentes específicos
│   ├── contexts/               # Contextos React (Auth, etc.)
│   ├── lib/                    # Utilitários e configurações
│   ├── services/               # Serviços de API
│   └── types/                  # Definições TypeScript
├── public/                     # Arquivos estáticos
├── components.json             # Configuração do shadcn/ui
├── tailwind.config.ts          # Configuração do Tailwind
└── next.config.ts              # Configuração do Next.js
```

## 🎨 Design System

O projeto utiliza um design system consistente baseado em:

- **Radix UI** - Componentes primitivos acessíveis
- **Tailwind CSS** - Utilitários CSS para estilização
- **Lucide React** - Ícones consistentes
- **CSS Variables** - Sistema de cores customizável

### Cores Principais

```css
--primary: 222.2 84% 4.9%
--secondary: 210 40% 98%
--accent: 210 40% 96%
--muted: 210 40% 96%
```

## 🔧 Scripts Disponíveis

```bash
# Desenvolvimento com Turbopack (mais rápido)
npm run dev

# Build para produção
npm run build

# Executar build de produção
npm run start

# Linting do código
npm run lint
```

## 🧪 Testando a Aplicação

```bash
# Verificar se o servidor está rodando
curl http://localhost:3000

# Ou abra no navegador
open http://localhost:3000
```

## 📚 Recursos Adicionais

- [Documentação do Next.js](https://nextjs.org/docs) - Aprenda sobre recursos do Next.js
- [Documentação do React](https://react.dev) - Guia oficial do React
- [Tailwind CSS](https://tailwindcss.com/docs) - Documentação do Tailwind
- [Radix UI](https://www.radix-ui.com) - Componentes acessíveis
