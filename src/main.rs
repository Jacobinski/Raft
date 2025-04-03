use crate::server::Server;

mod server;
mod watchdog;

fn main() {
    let server = Server::new();
    println!("{:?}", server)
}
