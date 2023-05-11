use honggfuzz::fuzz;
use bunt::{ write, termcolor::Buffer };

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(input_str) = std::str::from_utf8(data) {
                let mut buf = Buffer::ansi();
                write!(buf, "{$red+bold}{}{/$}", input_str).expect("failed to write to buffer in test");
                let _ = std::str::from_utf8(buf.as_slice()).expect("test produced non-UTF8 string").to_string();
            }
        });
    }
}