use crate::GLOBAL_DB;
use tracing;

// 连接数据库
pub async fn init_db() {
    tracing::info!("数据库连接");
    GLOBAL_DB.init(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:123456@localhost/ry-vue",
    )
        .unwrap();
}