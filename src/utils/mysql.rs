use crate::GLOBAL_DB;

// 连接数据库
pub async fn init_db() {
    GLOBAL_DB.init(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:123456@localhost/ry-vue",
    )
        .unwrap();
}