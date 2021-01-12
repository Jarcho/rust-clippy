#![warn(clippy::return_new_box)]
#![allow(clippy::needless_return, unused)]

trait Boxed: Sized {
    fn boxed(self) -> Box<Self>;
}
impl<T> Boxed for T {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

fn boxed(x: u32) -> Box<u32> {
    x.boxed()
}

fn box_new() -> Box<[u64; 4]> {
    Box::new([0; 4])
}

fn box_from() -> Box<[u64; 4]> {
    Box::from([0; 4])
}

fn from() -> Box<[u64; 4]> {
    From::from([0; 4])
}

fn into() -> Box<[u64; 4]> {
    [0; 4].into()
}

fn return_box_new() -> Box<[u64; 4]> {
    return Box::new([0; 4]);
}

fn return_cond(b: bool) -> Box<[u64; 4]> {
    if b {
        Box::new([0; 4])
    } else {
        panic!()
    }
}

fn main() {
    fn nested() -> Box<u32> {
        Box::from(0)
    }
}
