use local_ip_address::local_ip;

use crate::game::board::{self, Board};

use std::{collections::HashSet, net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket}};


pub struct Head {
    board: Board,
    socket: UdpSocket,
    clients: HashSet<SocketAddr>,
}

impl Head {
    pub fn new(local: bool) -> Self {
        let addr: SocketAddr; 
        if local {
            addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4509));
        } else {
            let device_ip = local_ip()
                .expect("Unable to get device ip address");
            addr = SocketAddr::new(device_ip, 4509);
        }

        let socket = UdpSocket::bind(addr).expect("Unable to connect to port");
        println!("Listening on port: {}", addr);
        Self {
            board: Board::new(),
            socket: socket,
            clients: HashSet::new()
        }
    }

    pub fn recv_loop(mut self) -> std::io::Result<()> {

        let mut buffer: [u8; 512] = [0u8; 512];        
        loop {
            let (bytes, addr) = self.socket.recv_from(&mut buffer)?;
            let message: String = String::from_utf8_lossy(&buffer[..bytes]).to_string();
            
            if (self.clients.contains(&addr))
            {
                if message == "QUIT" {
                    return Ok(());
                } else if message == "DISCONNECT" {
                    println!("Device disconnected. (IPAddr: {})", addr);
                    self.clients.remove(&addr);
                }
            } else if message == "CONNECT" {
                println!("Device connected (IPAddr: {})", addr);
                self.clients.insert(addr);
                self.socket.send_to(b"CHESSCONNECTED", addr)?;
            }
        }
    }
}