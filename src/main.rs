use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use sql_custom_types::models::User;
use sql_custom_types::non_empty_string::NonEmptyText;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;

fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool: DBPool = match r2d2::Pool::builder()
        // We set the maximum number of connections in the pool to 10
        .max_size(10)
        .build(manager)
    {
        Ok(client) => {
            log::info!("✅ Diesel connection to the database is successful!");
            client
        }
        Err(e) => {
            log::error!("🔥 Error connecting to the database with Diesel: {}", e);
            std::process::exit(1);
        }
    };
}
