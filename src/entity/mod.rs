use r2d2_mysql::mysql::Params;

pub trait Entity {
    fn table_name() -> &'static str;
    fn primary_key() -> &'static str;
    fn insert_sql() -> &'static str;
    fn update_by_id_sql(&self) -> String;
    fn params(&self) -> Params;
}