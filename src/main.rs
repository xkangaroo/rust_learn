mod config;  // 确保这里声明了config模块
mod routers;
mod handlers;

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use crate::config::{load_config};  // 导入config模块中的init和load_config
use tracing::{error, info, trace, Level};
use tracing_subscriber::FmtSubscriber;
use git_version::git_version;
#[tokio::main]
async fn main() {
    // 初始化日志记录器
    tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
    // 加载配置
    let config = load_config();
    let address = &config.server.address;
    let service_name = &config.server.name;

    // 创建主应用程序路由，加载所有子路由，并传递服务名称作为前缀
    let app = routers::create_routes(&service_name);


    const VERSION: &str = git_version!();
    println!("{VERSION}");

    // 创建监听器
    let listener = TcpListener::bind(address)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to bind to address {}: {}", address, e);
            panic!("Failed to bind to address {}: {}", address, e)
        }); 
    info!("启动服务,Listening on {}", address);
    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}
