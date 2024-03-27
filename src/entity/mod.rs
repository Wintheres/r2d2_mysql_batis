use r2d2_mysql::mysql::Params;

pub trait Entity {
    /// 获取表名
    fn table_name() -> &'static str;
    /// 获取主键名
    fn primary_key() -> &'static str;
    /// 获取插入JPA SQL语句
    fn insert_sql() -> &'static str;
    /// 获取根据主键更新JPA SQL语句（不包含None值的字段）
    fn update_by_id_sql(&self) -> String;
    /// 获取当前Entity对象的所有属性的键值对
    fn params(&self) -> Params;
}