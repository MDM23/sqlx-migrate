pub use sqlx_migrate_common::{Migration, MigrationError, Migrator};
pub use sqlx_migrate_macros::embed;

#[macro_export]
macro_rules! migrate {
    ($path: literal, $connection: expr) => {
        sqlx_migrate::embed!($path).migrate($connection)
    };
}
