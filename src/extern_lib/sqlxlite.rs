
use std::str::FromStr;
use std::time::{Duration, Instant};

use futures::*;



use sqlx::{Connection, Pool, sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous}, Sqlite};

pub async fn use_sqlx() -> Result<Vec<PenaltyMsg>, Box<dyn std::error::Error + 'static>> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:password@localhost/test").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool).await?;
    //
    // let database_file = "db.sqlite";
    // let database_url = format!("sqlite://db.sqlite", database_file);
    let pool_timeout = Duration::from_secs(30);

    let connection_options = SqliteConnectOptions::from_str("sqlite:///home/rui/penalty.db")?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(pool_timeout);

    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(6)
        .connect_timeout(pool_timeout)
        .connect_with(connection_options)
        .await?;
    // let _ = sqlite_pool.close().await;

    sqlx::query("CREATE TABLE if not exists  `penalty_msgs` (`to_addr` text,`from_addr` text,`height` integer,
                    `amount` text,`time_at` datetime,`call_function` text,`sub_cause` text);").execute(&sqlite_pool).await?;

    // sqlx::query("SELECT * from penalty_msg")
    //     .fetch_one(&sqlite_pool)
    //     .await?;

    let  pmsgs = sqlx::query_as::<_, PenaltyMsg>("SELECT * FROM penalty_msgs limit 3").fetch_all(&sqlite_pool).await?;

    println!("{}", pmsgs.len());
    println!("asfasdasdasd");
    for i in pmsgs.iter() {
        println!("{:?}", i);
    }

    let _  = sqlite_pool.close().await;

    return Ok(pmsgs);
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PenaltyMsg {
    pub to_addr: String,
    pub from_addr: String,
    pub height: i64,
    pub amount: String,
    pub time_at: String,
    pub call_function: String,
    pub sub_cause: String,
}