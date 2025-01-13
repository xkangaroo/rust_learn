mod config;  // 确保这里声明了config模块
mod routers;
mod handlers;

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use crate::config::{load_config};  // 导入config模块中的init和load_config
use tracing::{info, trace, error};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // 初始化日志记录器
    tracing::subscriber::set_global_default(
        FmtSubscriber::new()
    ).expect("setting default subscriber failed");
    // 加载配置an
    let config = load_config();

    let address = &config.server.address;
    let service_name = &config.server.name;

    // 创建主应用程序路由，加载所有子路由，并传递服务名称作为前缀
    let app = routers::create_routes(&service_name);
    
    
    // 创建监听器
    let listener = TcpListener::bind(address)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to address {}: {}", address, e));

    println!("Listening on {}", address);
    info!("启动服务");
    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}
