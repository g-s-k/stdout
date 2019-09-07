use std::{
    io::{self, ErrorKind, Write},
    process,
};

const HELLO: &[u8] = b"hello world\n";

fn main() {
    for _ in 0..200 {
        let mut std_out = io::stdout();
        match std_out.write_all(HELLO) {
            Err(ref e) if e.kind() == ErrorKind::BrokenPipe => process::exit(0),
            Err(e) => panic!("{:?}", e),
            Ok(_) => (),
        }
    }
}
