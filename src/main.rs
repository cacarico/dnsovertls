use std::net::UdpSocket;
use std::panic::resume_unwind;


const PACKET_BUFFER_SIZE: usize = 8192;

pub struct ServerOptions {
    pub bind_ip: String,
    pub bind_port: u16,
    pub worker_threads: u16,
}

fn start_server(bind_ip: String, bind_port: u16 ) {


    let bind_string = format!("{}:{}", bind_ip, bind_port);
    let socket = UdpSocket::bind(bind_string);
}

fn main() {

    let server = ServerOptions {
        bind_ip: String::from("127.0.0.1"),
        bind_port: 8080,
        worker_threads: 4
    };

    start_server(server.bind_ip, server.bind_port)
}
