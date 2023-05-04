// lazy_static! マクロで global な変数を定義する（遅延初期化）
// Mutex を使ってスレッドセーフにする

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("Called {}", ARRAY.lock().unwrap().len());
}
