# Portfolio Développeur Web

Un portfolio moderne utilisant Rust (backend) et Svelte (frontend).

## Architecture

- **Backend**: Rust avec Axum, SQLite, SQLx
- **Frontend**: SvelteKit avec TypeScript et TailwindCSS
- **Base de données**: SQLite
- **Développement**: Docker Compose

## Démarrage rapide

### Prérequis
- Docker et Docker Compose
- Node.js 18+ (pour développement local)
- Rust 1.75+ (pour développement local)

### Développement avec Docker

```bash
# Démarrer tous les services
docker-compose up --build

# Backend disponible sur: http://localhost:3001
# Frontend disponible sur: http://localhost:5174
```

### Développement local

#### Backend
```bash
cd backend
cargo run
```

#### Frontend
```bash
cd frontend
npm install
npm run dev
```

## Structure du projet

```
├── backend/           # API Rust avec Axum
├── frontend/          # Application SvelteKit
├── data/             # Base de données SQLite
├── docker-compose.yml # Configuration Docker
└── README.md
```

## API Endpoints

- `GET /api/projects` - Liste des projets
- `GET /api/skills` - Liste des compétences
- `GET /api/profile` - Informations du profil
- `POST /api/contact` - Formulaire de contact

## Fonctionnalités

- ✅ Architecture moderne Rust + Svelte
- ✅ Interface responsive avec TailwindCSS
- ✅ API REST performante
- ✅ Base de données SQLite
- ✅ Configuration Docker pour le développement
- 🚧 Interface d'administration
- 🚧 Système d'authentification
- 🚧 Upload d'images