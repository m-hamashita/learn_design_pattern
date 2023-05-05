mod fs;

use fs::{Component, File, Folder};

// File と Folder はどちらも Component traitを実装している
// Folder の search は再帰的に呼び出される
// File: leaf, Folder: composite(node)
fn main() {
    let file1 = File::new("File 1");
    let file2 = File::new("File 2");
    let file3 = File::new("File 3");

    let mut folder1 = Folder::new("Folder 1");
    folder1.add(file1);

    let mut folder2 = Folder::new("Folder 2");
    folder2.add(file2);
    folder2.add(file3);
    folder2.add(folder1);

    folder2.search("rose");
}
