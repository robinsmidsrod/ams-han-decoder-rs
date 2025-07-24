use std::{io::Read, net::TcpStream};

pub use self::error::{Error, Result};

use clap::Parser;
use wild::ArgsOs;

mod error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Stream to connect to
    ///
    /// Format: hostname:port
    #[arg()]
    host_and_port: String,
}

pub fn run(args: ArgsOs) -> Result<()> {
    let args = Args::parse_from(args);
    println!("{args:?}");

    let mut stream = TcpStream::connect(args.host_and_port)?;
    loop {
        let mut buf = vec![0; 1024];
        match stream.read(&mut buf) {
            Ok(n) => {
                buf.resize(n, 0);
                hexdump(&buf);
            }
            Err(e) => {
                eprintln!("Reading from socket failed: {e}");
                break;
            }
        }
    }
    Ok(())
}

fn hexdump(buf: &[u8]) {
    let n = buf.len();
    for &byte in buf.iter() {
        print!("{:0>2x} ", byte);
    }
    print!("{}|", " ".repeat((12 - n) * 3));
    for &byte in buf.iter() {
        let c = if (32..128).contains(&byte) {
            char::from_u32(byte as u32).unwrap()
        } else {
            '.'
        };
        print!("{c}");
    }
    print!("{}|", " ".repeat(12 - n));
    print!(" len={:0>2}", n);
    println!();
}
