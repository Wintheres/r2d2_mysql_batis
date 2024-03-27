use r2d2_mysql::mysql::{params, Params};
use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use tokio::task;
use crate::config::mysql;
use crate::entity::Entity;

pub type BoxErr = Box<dyn std::error::Error + Send + Sync>;

pub trait Service: Send + Sized + Entity + FromRow + 'static {
    /// 设置主键值
    fn set_primary_key(&mut self, id: u64);

    /// 自定义SQL查询
    async fn query(query_sql: String) -> Result<Vec<Self>, BoxErr> {
        let mut coon = mysql::get_coon().await?;
        task::spawn_blocking(move || {
            let mut vec = vec![];
            let _ = coon.query(query_sql)
                .map(|rows: Vec<Self>| {
                    for row in rows {
                        vec.push(row);
                    }
                })?;
            Ok(vec)
        }).await?
    }

    /// 自定义JPA SQL查询
    async fn jpa_query(query_sql: String, params: Params) -> Result<Vec<Self>, BoxErr> {
        let mut coon = mysql::get_coon().await?;
        task::spawn_blocking(move || {
            let mut vec = vec![];
            let _ = coon.exec(query_sql, params)
                .map(|rows: Vec<Self>| {
                    for row in rows {
                        vec.push(row);
                    }
                })?;
            Ok(vec)
        }).await?
    }

    /// 获取列表
    async fn list() -> Result<Vec<Self>, BoxErr> {
        Self::query(format!("SELECT * FROM `{}` ORDER BY `{}` ASC", Self::table_name(), Self::primary_key())).await
    }

    /// 根据主键单条查询
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

    /// 将当前Entity对象插入数据库
    async fn add(mut self) -> Result<Self, BoxErr> {
        let mut coon = mysql::get_coon().await?;
        task::spawn_blocking(move || {
            let _result: Vec<String> = coon.exec(Self::insert_sql(), self.params())?;
            self.set_primary_key(coon.last_insert_id());
            Ok(self)
        }).await?
    }

    /// 将当前Entity对象根据主键进行数据更新（不包含None值的字段）
    async fn edit(self) -> Result<Self, BoxErr> {
        let mut coon = mysql::get_coon().await?;
        task::spawn_blocking(move || {
            let _result: Vec<String> = coon.exec(self.update_by_id_sql(), self.params())?;
            Ok(self)
        }).await?
    }

    /// 根据主键单条删除
    async fn remove(id: u64) -> Result<(), BoxErr> {
        let mut coon = mysql::get_coon().await?;
        task::spawn_blocking(move || {
            let _result: Vec<String> = coon.exec(format!("DELETE FROM `{}` WHERE `{}` = :id", Self::table_name(), Self::primary_key()), params! {"id" => id})?;
            Ok(())
        }).await?
    }
}