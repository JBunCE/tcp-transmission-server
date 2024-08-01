use tokio::net::TcpStream;

pub struct Client {
    pub socket: TcpStream,
    pub address: String,
}