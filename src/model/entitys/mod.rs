pub mod ytb_entity;
pub mod pagedata;
pub mod crawler_entity;

use rbatis::DateTimeNative;
/**
*struct:CommonField
*desc:所有表的公共字段 CRUD_SERVICE使用
*/
#[derive(Clone, Debug)]
pub struct CommonField {
    pub id: Option<i64>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}