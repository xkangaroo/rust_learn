mod config;  // 确保这里声明了config模块
mod routers;
mod handlers;
mod common;

use common::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use version_info::VersionInfoProvider;
use crate::config::{load_config};  // 导入config模块中的init和load_config
use tracing::{error, info, trace, Level};
use tracing_subscriber::FmtSubscriber;


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

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "-v" {
        let provider = version_info::GitVersionInfoProvider; // 确保这里的名称与定义一致
        match provider.get_version_info() {
            Ok(version_info) => version_info.print(),
            Err(e) => eprintln!("Error getting version info: {}", e),
        }
        return;
    }
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
