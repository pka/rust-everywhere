extern crate rust_everywhere;

use rust_everywhere::build_info;
use rust_everywhere::pgtest;

fn main() {
    println!("{}", build_info::BUILD_INFO);
    pgtest::pgquery();
}

