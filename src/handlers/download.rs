use axum::{extract::Path, Router, routing::get, response::Response};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::body::Body; // 引入 Body 类型
use include_dir::{include_dir, Dir};
use bytes::Bytes;
use tracing::{info, trace, error};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub async fn get_download_handler(Path(encoded_file_name): Path<String>) -> Response<Body> {
    // 查找指定文件
    if let Some(file) = TEMPLATES_DIR.get_file(&encoded_file_name) {
        let file_bytes = Bytes::copy_from_slice(file.contents());

        // 构造响应，设置文件下载的头信息
        let mut response = Response::new(Body::from(file_bytes)); // 使用 Body::from 包装 Bytes
        response.headers_mut().insert(
            CONTENT_TYPE,
            "application/octet-stream".parse().unwrap(),
        );
        response.headers_mut().insert(
            CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", encoded_file_name).parse().unwrap(),
        );
        info!(target: "yak_events", "Commencing yak shaving for");
        response
    } else {
        // 如果文件没有找到，返回 404 错误
        Response::builder()
            .status(404)
            .body(Body::from("File not found")) // 使用 Body::from 包装错误消息
            .unwrap()
    }
}
