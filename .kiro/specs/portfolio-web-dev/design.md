# Design Document - Portfolio Développeur Web

## Overview

Le portfolio utilise une architecture moderne séparant clairement le frontend et le backend. Rust fournit une API REST performante et sécurisée, tandis que Svelte offre une interface utilisateur réactive et légère. L'application est conçue pour être facilement déployable et maintenable.

## Architecture

### Architecture Générale

```
┌─────────────────┐    HTTP/REST    ┌──────────────────┐
│   Frontend      │ ◄──────────────► │    Backend       │
│   (Svelte)      │                  │    (Rust)        │
│                 │                  │                  │
│ - SvelteKit     │                  │ - Axum/Warp      │
│ - TailwindCSS   │                  │ - SQLite         │
│ - TypeScript    │                  │ - Serde          │
└─────────────────┘                  └──────────────────┘
                                              │
                                              ▼
                                     ┌──────────────────┐
                                     │   Base de        │
                                     │   Données        │
                                     │   (SQLite)       │
                                     └──────────────────┘
```

### Stack Technologique

**Backend (Rust)**
- **Framework Web**: Axum pour sa performance et sa simplicité
- **Base de données**: SQLite avec SQLx pour la persistance
- **Sérialisation**: Serde pour JSON
- **Validation**: Validator pour les données d'entrée
- **CORS**: Tower-http pour la gestion des requêtes cross-origin
- **Logging**: Tracing pour le monitoring

**Frontend (Svelte)**
- **Framework**: SvelteKit pour le SSR et le routing
- **Styling**: TailwindCSS pour un design moderne et responsive
- **Animations**: Svelte transitions + Framer Motion
- **HTTP Client**: Fetch API native
- **Build**: Vite pour un développement rapide

## Components and Interfaces

### Backend Components

#### 1. API Routes Structure
```
/api/
├── /projects          # GET, POST, PUT, DELETE
├── /skills           # GET, POST, PUT, DELETE  
├── /profile          # GET, PUT
├── /contact          # POST (envoi email)
└── /admin            # Authentication routes
```

#### 2. Data Models

**Project Model**
```rust
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub long_description: Option<String>,
    pub technologies: Vec<String>, // JSON array
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
    pub image_url: Option<String>,
    pub category: String,
    pub featured: bool,
    pub created_at: DateTime<Utc>,
}
```

**Skill Model**
```rust
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub category: String, // Frontend, Backend, Tools, etc.
    pub level: i32,       // 1-5 scale
    pub years_experience: Option<i32>,
    pub description: Option<String>,
}
```

**Profile Model**
```rust
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub bio: String,
    pub email: String,
    pub phone: Option<String>,
    pub location: String,
    pub avatar_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
    pub twitter_url: Option<String>,
}
```

#### 3. Service Layer
```rust
pub struct ProjectService {
    db: Arc<SqlitePool>,
}

impl ProjectService {
    pub async fn get_all_projects(&self) -> Result<Vec<Project>>;
    pub async fn get_featured_projects(&self) -> Result<Vec<Project>>;
    pub async fn create_project(&self, project: CreateProject) -> Result<Project>;
    pub async fn update_project(&self, id: i32, project: UpdateProject) -> Result<Project>;
    pub async fn delete_project(&self, id: i32) -> Result<()>;
}
```

### Frontend Components

#### 1. Page Structure
```
src/
├── routes/
│   ├── +layout.svelte          # Layout principal
│   ├── +page.svelte            # Page d'accueil
│   ├── projects/
│   │   ├── +page.svelte        # Liste des projets
│   │   └── [id]/+page.svelte   # Détail projet
│   ├── about/+page.svelte      # À propos
│   ├── contact/+page.svelte    # Contact
│   └── admin/
│       ├── +layout.svelte      # Layout admin
│       ├── +page.svelte        # Dashboard
│       ├── projects/+page.svelte
│       └── skills/+page.svelte
├── lib/
│   ├── components/
│   │   ├── Header.svelte
│   │   ├── Footer.svelte
│   │   ├── ProjectCard.svelte
│   │   ├── SkillBadge.svelte
│   │   └── ContactForm.svelte
│   ├── stores/
│   │   ├── projects.ts
│   │   ├── skills.ts
│   │   └── profile.ts
│   └── api/
│       └── client.ts           # API client
└── app.html
```

#### 2. Key Components

**ProjectCard Component**
```svelte
<script lang="ts">
  export let project: Project;
  export let featured = false;
</script>

<div class="project-card" class:featured>
  <img src={project.image_url} alt={project.title} />
  <div class="content">
    <h3>{project.title}</h3>
    <p>{project.description}</p>
    <div class="technologies">
      {#each project.technologies as tech}
        <span class="tech-badge">{tech}</span>
      {/each}
    </div>
    <div class="actions">
      {#if project.demo_url}
        <a href={project.demo_url} target="_blank">Démo</a>
      {/if}
      {#if project.github_url}
        <a href={project.github_url} target="_blank">Code</a>
      {/if}
    </div>
  </div>
</div>
```

## Data Models

### Database Schema

```sql
-- Profile table
CREATE TABLE profile (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    title TEXT NOT NULL,
    bio TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    location TEXT NOT NULL,
    avatar_url TEXT,
    linkedin_url TEXT,
    github_url TEXT,
    twitter_url TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Projects table
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    long_description TEXT,
    technologies TEXT NOT NULL, -- JSON array
    github_url TEXT,
    demo_url TEXT,
    image_url TEXT,
    category TEXT NOT NULL,
    featured BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Skills table
CREATE TABLE skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    level INTEGER NOT NULL CHECK (level >= 1 AND level <= 5),
    years_experience INTEGER,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Contact messages table
CREATE TABLE contact_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    subject TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### API Response Formats

**Success Response**
```json
{
  "success": true,
  "data": { ... },
  "message": "Operation completed successfully"
}
```

**Error Response**
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input data",
    "details": ["Email is required", "Name must be at least 2 characters"]
  }
}
```

## Error Handling

### Backend Error Handling
```rust
#[derive(Debug, Serialize)]
pub enum ApiError {
    DatabaseError(String),
    ValidationError(Vec<String>),
    NotFound(String),
    Unauthorized,
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ValidationError(errors) => (StatusCode::BAD_REQUEST, errors.join(", ")),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "success": false,
            "error": {
                "message": error_message
            }
        }));

        (status, body).into_response()
    }
}
```

### Frontend Error Handling
- Toast notifications pour les erreurs utilisateur
- Fallback UI pour les erreurs de chargement
- Retry automatique pour les erreurs réseau
- Logging des erreurs côté client

## Testing Strategy

### Backend Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_project() {
        // Test de création de projet
    }
    
    #[tokio::test]
    async fn test_get_projects_by_category() {
        // Test de filtrage par catégorie
    }
}
```

**Types de tests Backend:**
- Tests unitaires pour les services et modèles
- Tests d'intégration pour les routes API
- Tests de base de données avec SQLite en mémoire
- Tests de validation des données

### Frontend Testing
```javascript
// tests/ProjectCard.test.ts
import { render, screen } from '@testing-library/svelte';
import ProjectCard from '$lib/components/ProjectCard.svelte';

test('displays project information correctly', () => {
  const project = {
    title: 'Test Project',
    description: 'Test description',
    technologies: ['Rust', 'Svelte']
  };
  
  render(ProjectCard, { project });
  
  expect(screen.getByText('Test Project')).toBeInTheDocument();
  expect(screen.getByText('Rust')).toBeInTheDocument();
});
```

**Types de tests Frontend:**
- Tests unitaires des composants Svelte
- Tests d'intégration des pages
- Tests E2E avec Playwright
- Tests de responsive design

### Stratégie de déploiement
- **Développement**: Docker Compose avec hot reload
- **Production**: Conteneurs séparés pour frontend et backend
- **CI/CD**: GitHub Actions pour tests automatisés et déploiement
- **Monitoring**: Logs structurés et métriques de performance