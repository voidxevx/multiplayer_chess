pub mod game {
    pub mod board;
    pub mod server_head;
}

use std::env;

use crate::game::{board::Board, server_head::{self, Head}};


fn main() -> std::io::Result<()> {
    let as_local = match env::args().nth(1) {
        Some(n) => {
            n == "local"   
        }
        _ => false,
    };

    let head: Head = Head::new(as_local);
    head.recv_loop()
}
