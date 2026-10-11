# Gamers CM - Frontend

A modern web application built with Next.js for gaming community management. Intuitive and responsive interface for managing communities, players, teams, and gaming events.

## 🚀 Technologies Used

- **Next.js 15** - React framework for modern web applications
- **React 19** - Library for user interfaces
- **TypeScript** - JavaScript superset with static typing
- **Tailwind CSS** - Utility-first CSS framework for responsive design
- **Radix UI** - Accessible and customizable components
- **Axios** - HTTP client for API communication
- **Lucide React** - Modern and consistent icons

## ✨ Features

- **User Authentication** - Secure login and registration
- **Community Management** - Create and administer gaming communities
- **Player System** - Player profiles and member management
- **Team System** - Team formation and management within communities
- **Event Management** - Create and manage matches and tournaments
- **Responsive Interface** - Design adaptable for desktop and mobile
- **Dark/Light Theme** - Support for dark and light modes
- **Reusable Components** - Consistent UI component library

## 📋 Prerequisites

Before you begin, make sure you have installed:

- **Node.js** (version 18 or higher)
- **npm** or **yarn** or **pnpm**
- **Git**

## 🛠️ Setup and Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd gamer-cm/gamer-cm-front
```

### 2. Install Dependencies

```bash
# Using npm
npm install

# Or using yarn
yarn install

# Or using pnpm
pnpm install
```

### 3. Configure Environment Variables

Create a `.env.local` file in the project root:

```env
# Backend API URL
NEXT_PUBLIC_API_URL=http://localhost:8080

# Environment
NODE_ENV=development
```

### 4. Run the Project

```bash
# Development mode
npm run dev

# Or using yarn
yarn dev

# Or using pnpm
pnpm dev
```

## 🚀 Running the Project

### Development

```bash
# Terminal 1: Run the backend (gamer-cm-api)
cd ../gamer-cm-api
cargo run

# Terminal 2: Run the frontend
cd gamer-cm-front
npm run dev
```

The application will be available at: `http://localhost:3000`

### Success Logs

When everything is working correctly, you will see:

```
✓ Ready in 2.3s
✓ Local:        http://localhost:3000
✓ Network:      http://192.168.1.100:3000
```

## 📁 Project Structure

```
gamer-cm-front/
├── src/
│   ├── app/                    # Pages and layouts (App Router)
│   │   ├── (authenticated)/    # Protected routes
│   │   ├── login/              # Login page
│   │   └── register/           # Registration page
│   ├── components/             # Reusable components
│   │   ├── ui/                 # Base components (Radix UI)
│   │   └── *.tsx               # Specific components
│   ├── contexts/               # React contexts (Auth, etc.)
│   ├── lib/                    # Utilities and configurations
│   ├── services/               # API services
│   └── types/                  # TypeScript definitions
├── public/                     # Static files
├── components.json             # shadcn/ui configuration
├── tailwind.config.ts          # Tailwind configuration
└── next.config.ts              # Next.js configuration
```

## 🎨 Design System

The project uses a consistent design system based on:

- **Radix UI** - Accessible primitive components
- **Tailwind CSS** - CSS utilities for styling
- **Lucide React** - Consistent icons
- **CSS Variables** - Customizable color system

### Main Colors

```css
--primary: 222.2 84% 4.9%
--secondary: 210 40% 98%
--accent: 210 40% 96%
--muted: 210 40% 96%
```

## 🔧 Available Scripts

```bash
# Development with Turbopack (faster)
npm run dev

# Build for production
npm run build

# Run production build
npm run start

# Code linting
npm run lint
```

## 🧪 Testing the Application

```bash
# Check if the server is running
curl http://localhost:3000

# Or open in browser
open http://localhost:3000
```

## 📚 Additional Resources

- [Next.js Documentation](https://nextjs.org/docs) - Learn about Next.js features
- [React Documentation](https://react.dev) - Official React guide
- [Tailwind CSS](https://tailwindcss.com/docs) - Tailwind documentation
- [Radix UI](https://www.radix-ui.com) - Accessible components
