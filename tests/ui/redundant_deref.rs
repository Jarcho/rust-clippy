#![warn(clippy::redundant_deref)]

fn foo(_: &str) {}

fn main() {
    foo(&*String::new());
}
