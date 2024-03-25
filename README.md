# Overview

### 这是一个模仿mybatis的简单curd，目前仅支持mysql数据库。

### 如何使用？

#### 第一步：
在项目根目录下添加db.yml配置文件，其中编写数据库相关配置。
```
host: 127.0.0.1
port: 3306
user: root
password: 123456
db-name: test
```

#### 第二步：
假设你有一张`sys_user`表，其中主键名为`user_id`，那么在实体struct中， 
需要添加一个`entity_option_mapping`的属性宏，该宏接收两个属性参数， 第一个是表名，第二个是主键名。
所有字段必须使用Option类型，且泛型的类型是基本类型或String。
```
use r2d2_mysql_batis::macros::entity_option_mapping;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[entity_option_mapping(sys_user, user_id)]
pub struct SysUser {
    pub user_id: Option<u64>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub dept_id: Option<u64>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub status: Option<i8>,
    pub user_id_create: Option<u64>,
    pub gmt_create: Option<String>,
    pub gmt_modified: Option<String>,
    pub sex: Option<i64>,
    pub birth: Option<String>,
    pub pic_id: Option<i64>,
    pub live_address: Option<String>,
    pub hobby: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
}
```

#### 第三步：
```
use crate::entity::SysUser;
use r2d2_mysql_batis::entity::Entity;
use r2d2_mysql_batis::service::Service;

#[tokio::test]
async fn tokio_test() {
        let mut user = SysUser::find_by_id(1).await.unwrap().unwrap();
        println!("{user:?}");

        // 插入数据库
        user.add().await;

        // 根据主键更新（None值字段忽略）
        user.edit().await;

        // 根据主键删除
        SysUser::remove(999999).await;

        // 获取表名 返回类型为&str
        let table_name = SysUser::table_name();
        println!("{table_name}");

        // 获取主键名 返回类型为&str
        let primary_key = SysUser::primary_key();
        println!("{primary_key}");

        // 获取插入SQL语句 返回类型为&str
        let insert_sql = SysUser::insert_sql();
        println!("{insert_sql}");

        // 获取根据主键更新SQL语句（更新字段不包含None值）返回类型为String
        let update_by_id_sql = user.update_by_id_sql();
        println!("{update_by_id_sql}");

        //设置主键值（数据库不更新）
        user.set_primary_key(1);
}
```