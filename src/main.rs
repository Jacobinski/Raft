use crate::server::Server;

mod server;

fn main() {
    let server = Server::new();
    println!("{:?}", server)
}
