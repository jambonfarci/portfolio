// Database module
pub mod connection;
pub mod migrations;
pub mod seed;
pub mod init;
pub mod repositories;

pub use connection::{DatabaseConfig, create_pool, test_connection};
pub use migrations::{MigrationManager, initialize_database, MigrationError};
pub use seed::{seed_database, SeedError};
pub use init::{initialize_complete_database, initialize_test_database, InitError};
pub use repositories::{ProjectRepository, SkillRepository, ProfileRepository, ContactRepository};