use std::{
    io::{self, ErrorKind, Write},
    process,
};

macro_rules! print_safe {
    ($fmt:expr $(, $e:expr)*) => {{
        let mut std_out = io::stdout();
        match std_out.write_fmt(format_args!($fmt $(, $e:expr)*)) {
            Err(ref e) if e.kind() == ErrorKind::BrokenPipe => process::exit(0),
            Err(e) => panic!("{:?}", e),
            Ok(_) => (),
        }
    }}
}

fn main() {
    for _ in 0..200 {
        print_safe!("hello world\n");
    }
}
