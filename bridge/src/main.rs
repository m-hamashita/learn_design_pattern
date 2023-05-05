mod device;
mod remotes;

use device::{Device, Radio, Tv};
use remotes::{AdvancedRemove, BasicRemote, HasMutableDevice, Remote};

fn main() {
    // 異なる実装（デバイスの追加等）は共通のインターフェース（Device）に従っている限り交換が可能
    // 具体的なデバイスは Device trait を実装することで追加できる
    // Remote は Device に依存している
    // Device trait が実装されている限り、どのようなデバイスでも Remote に接続できる
    // つまり Device と Remote は独立して拡張できる
    test_device(Tv::default());
    test_device(Radio::default());
}

fn test_device(device: impl Device + Clone) {
    println!("Tests with basic remote.");
    let mut basic_remote = BasicRemote::new(device.clone());
    basic_remote.power();
    basic_remote.device().print_status();

    println!("Tests with advanced remote.");
    let mut advanced_remote = AdvancedRemove::new(device);
    advanced_remote.power();
    advanced_remote.mute();
    advanced_remote.device().print_status();
}
