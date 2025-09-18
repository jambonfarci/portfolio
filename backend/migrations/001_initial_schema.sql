-- Initial database schema for portfolio application

-- Profile table - stores developer profile information
CREATE TABLE IF NOT EXISTS profile (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    title TEXT NOT NULL,
    bio TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    location TEXT NOT NULL,
    linkedin_url TEXT,
    github_url TEXT,
    twitter_url TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Projects table - stores portfolio projects
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    long_description TEXT,
    technologies TEXT NOT NULL, -- JSON array as string
    github_url TEXT,
    demo_url TEXT,
    image_url TEXT,
    category TEXT NOT NULL,
    featured BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Skills table - stores technical skills and competencies
CREATE TABLE IF NOT EXISTS skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    level INTEGER NOT NULL CHECK (level >= 1 AND level <= 5),
    years_experience INTEGER,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Contact messages table - stores messages from contact form
CREATE TABLE IF NOT EXISTS contact_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    subject TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Insert default profile data
INSERT OR IGNORE INTO profile (id, name, title, bio, email, location, linkedin_url, github_url, twitter_url) VALUES (
    1,
    'John Doe',
    'Full Stack Developer',
    'Passionate developer with expertise in modern web technologies including Rust, TypeScript, and cloud infrastructure. I love building scalable applications and exploring new technologies.',
    'john.doe@example.com',
    'Paris, France',
    'https://linkedin.com/in/johndoe',
    'https://github.com/johndoe',
    'https://twitter.com/johndoe'
);