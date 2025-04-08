mod server;

use server::server::Server;
fn main() {
    let server = Server::new("127.0.0.1", "7000");
    server.run();
}
