use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    // 创建50字节的buffer
    let mut data = [0 as u8; 50]; 
    // 错误匹配
    match stream.read(&mut data) {
        Ok(size) => {
            // 返回所有接收到信息
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            // 出错后 打印错误并退出
            println!("An error occurred, terminating connection with {}", stream.peer_addr()?);
            // 关闭stream
            stream.shutdown(Shutdown::Both)?;
            false
        }
    } 
}

fn main() {
    // 监听端口，unwrap取出Some值，空值panic
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    // accept connections and process them, spawning a new thread for each one 
    for stream in listener.incoming() {
        // 模式匹配，处理错误和异常
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                //  创建一个线程处理 ，move 强制闭包获取其使用的值的所有权
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}