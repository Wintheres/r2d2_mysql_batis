use r2d2_mysql::mysql::params;
use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use tokio::task;
use crate::config::mysql;
use crate::entity::Entity;

pub type BoxErr = Box<dyn std::error::Error + Send + Sync>;

pub trait Service: Send + Sized + Entity + FromRow + 'static {
    fn set_primary_key(&mut self, id: u64);

    async fn find_by_id(id: u64) -> Result<Option<Self>, BoxErr> {
        let mut coon = mysql::get_coon().await?;
        task::spawn_blocking(move || {
            let result = coon.exec_first(format!("SELECT * FROM `{}` WHERE `{}` = :id LIMIT 1", Self::table_name(), Self::primary_key()), params! {"id" => id})?;
            if let Some(row) = result {
                let vo = Self::from_row(row);
                return Ok(Some(vo));
            }
            Ok(None)
        }).await?
    }

    async fn add(mut self) -> Result<Self, BoxErr> {
        let mut coon = mysql::get_coon().await?;
        task::spawn_blocking(move || {
            let _result: Vec<String> = coon.exec(Self::insert_sql(), self.params())?;
            self.set_primary_key(coon.last_insert_id());
            Ok(self)
        }).await?
    }

    async fn edit(self) -> Result<Self, BoxErr> {
        let mut coon = mysql::get_coon().await?;
        task::spawn_blocking(move || {
            let _result: Vec<String> = coon.exec(self.update_by_id_sql(), self.params())?;
            Ok(self)
        }).await?
    }

    async fn remove(id: u64) -> Result<(), BoxErr> {
        let mut coon = mysql::get_coon().await?;
        task::spawn_blocking(move || {
            let _result: Vec<String> = coon.exec(format!("DELETE FROM `{}` WHERE `{}` = :id", Self::table_name(), Self::primary_key()), params! {"id" => id})?;
            Ok(())
        }).await?
    }
}