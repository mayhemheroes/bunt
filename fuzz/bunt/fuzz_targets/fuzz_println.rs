use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(input_str) = std::str::from_utf8(data) {
                bunt::println!("{[green]:?}, {[blue]:?}, {[red]:?}, {[yellow]:?}, {[white]:?}", input_str, input_str, input_str, input_str, input_str);
                bunt::println!("{[green]:?}. {$bold}Length: {[cyan]}{/$}", input_str, input_str.len());
            }
        });
    }
}