pub mod mysql {
    use std::error::Error;
    use once_cell::sync::Lazy;

    use r2d2_mysql::{mysql::OptsBuilder, r2d2, MySqlConnectionManager};
    use r2d2_mysql::r2d2::PooledConnection;
    use tokio::task;

    static POOL: Lazy<r2d2::Pool<MySqlConnectionManager>> = Lazy::new(|| {
        let builder = OptsBuilder::new()
            .ip_or_hostname(Some("127.0.0.1"))
            .tcp_port(3306)
            .user(Some("user"))
            .pass(Some("password"))
            .db_name(Some("db_name"));
        let manager = MySqlConnectionManager::new(builder);
        r2d2::Pool::builder().max_size(5).build(manager).unwrap()
    });

    pub async fn get_coon() -> Result<PooledConnection<MySqlConnectionManager>, Box<dyn Error + Send + Sync>> {
        task::spawn_blocking(move || {
            Ok(POOL.clone().get()?)
        }).await?
    }
}