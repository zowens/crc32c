extern crate gcc;

use gcc::Build;

fn main() {
    let mut cc = Build::new();
    cc.file("crc32c.c");
    cc.warnings_into_errors(false).compile("crc32");
}
