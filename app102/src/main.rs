use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.write(b"hello world\n")?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        handle_client(&mut stream?)?;
    }
    Ok(())
}
/*
$ nc 127.0.0.1 8080

hello world
 */
