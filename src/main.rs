mod server;

use librarius::{init, Config, Level};
use server::server::Server;
fn main() {
    let config = Config::with_file(Level::Debug, "./server.log");
    init(config);
    let server = Server::new("127.0.0.1", "7000");
    server.run();
}
