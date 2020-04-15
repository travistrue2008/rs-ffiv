mod common;
mod error;
mod lzss;
mod metadata;
mod unpacker;

fn main() {
    unpacker::unpack("assets/PAC0.BIN", "assets/PAC1.BIN", "assets/data").unwrap();
}
