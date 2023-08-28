use std::fs;
use async_std::task;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

#[async_std::main]
pub async fn loop_listen() {
    // 监听本地端口 7878 ，等待 TCP 连接的建立
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 警告，这里无法并发
        handle_connection(stream).await;
    }
}

