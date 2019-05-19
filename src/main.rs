use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::io;

fn main() {
    let address = "127.0.0.1:11211".parse().unwrap();
    let listener = TcpListener::bind(&address).unwrap();

    let server = listener.incoming().for_each(|mut socket| {
        let mut buff: [u8;255] = [0;255];
        let r = io::read(socket, buff);

        tokio::spawn(r);

        println!("read buffer: {}", String::from_utf8_lossy(&buff));
        Ok(())
    })
    .map_err(|err| {
        println!("accept error = {}", err);
    });


    tokio::run(server);
}
