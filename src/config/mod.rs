pub mod mysql {
    extern crate yaml_rust;
    use yaml_rust::YamlLoader;
    use std::error::Error;
    use std::fs::File;
    use std::io::Read;
    use once_cell::sync::Lazy;

    use r2d2_mysql::{mysql::OptsBuilder, r2d2, MySqlConnectionManager};
    use r2d2_mysql::r2d2::PooledConnection;
    use tokio::task;

    static POOL: Lazy<r2d2::Pool<MySqlConnectionManager>> = Lazy::new(|| {
        let mut file = File::open("./db.yml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
        let doc = &docs[0];
        let builder = OptsBuilder::new()
            .ip_or_hostname(Some(doc["host"].as_str().unwrap()))
            .tcp_port(doc["port"].as_i64().unwrap() as u16)
            .user(Some(doc["user"].as_str().unwrap()))
            .pass(Some(doc["password"].as_str().unwrap()))
            .db_name(Some(doc["db-name"].as_str().unwrap()));
        let manager = MySqlConnectionManager::new(builder);
        r2d2::Pool::builder().max_size(5).build(manager).unwrap()
    });

    pub async fn get_coon() -> Result<PooledConnection<MySqlConnectionManager>, Box<dyn Error + Send + Sync>> {
        task::spawn_blocking(move || {
            Ok(POOL.clone().get()?)
        }).await?
    }
}