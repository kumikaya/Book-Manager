pub use sea_orm_migration::prelude::*;

mod versions;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(versions::m001_create_books_table::Migration),
            Box::new(versions::m002_create_users_table::Migration),
            Box::new(versions::m003_create_borrowed_books_table::Migration),
            Box::new(versions::m004_create_emails_table::Migration),
        ]
    }
}
