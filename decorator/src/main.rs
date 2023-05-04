use std::io::{BufReader, Cursor, Read};

fn main() {
    // buffer の作成
    let mut buf = [0u8; 10];

    // pub fn new(inner: R) -> BufReader<R> {
    //     BufReader::with_capacity(DEFAULT_BUF_SIZE, inner)
    // }
    // BufReader::new でバイト列を decorate する
    // Unicode 文字列: UTF-8 でエンコードされたバイト列（1byte から 4byte）
    // バイト列: 8bit の固定長のバイト列
    let mut input = BufReader::new(Cursor::new("Input data"));

    // buffering された reader から読み込み
    input.read(&mut buf).ok();

    print!("Read from a buffered reader: ");

    for byte in buf {
        print!("{}", char::from(byte));
    }

    println!();
}
