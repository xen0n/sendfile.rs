#![feature(core)]
extern crate core;

extern crate sendfile;

use core::str::FromStr;


fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 {
        println!("usage: {} <source-file> <host:port>", args[0]);
        return;
    }

    let ref src_path = args[1];
    let ref dest_addr_str = args[2];
    let dest_addr = std::net::SocketAddr::from_str(unsafe { dest_addr_str.slice_unchecked(0, dest_addr_str.len()) }).unwrap();

    let mut infile = std::fs::File::open(src_path).unwrap();
    let mut stream = std::net::TcpStream::connect(dest_addr).unwrap();

    let bytes_written = sendfile::sendfile_all(&infile, &stream, 4096).unwrap();
    println!("Ok -- {} byte(s) written", bytes_written);
}
