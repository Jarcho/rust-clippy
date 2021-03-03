// run-rustfix

#![allow(clippy::needless_borrowed_reference)]

fn x(y: &i32) -> i32 {
    *y
}

#[warn(clippy::all, clippy::needless_borrow)]
#[allow(unused_variables)]
fn main() {
    let a = 5;
    let b = x(&a);
    let c = x(&&a);
    let s = &String::from("hi");
    let s_ident = f(&s); // should not error, because `&String` implements Copy, but `String` does not
    let g_val = g(&Vec::new()); // should not error, because `&Vec<T>` derefs to `&[T]`
    let vec = Vec::new();
    let vec_val = g(&vec); // should not error, because `&Vec<T>` derefs to `&[T]`
    h(&"foo"); // should not error, because the `&&str` is required, due to `&Trait`
    if let Some(ref cake) = Some(&5) {}
    let ref_a = &a;
    let garbl = match 42 {
        44 => &a,
        45 => {
            println!("foo");
            &&a
        },
        46 => &&a,
        #[allow(clippy::never_loop)]
        47 => loop {
            break &ref_a;
        },
        48 =>
        {
            #[allow(clippy::never_loop)]
            loop {
                break &ref_a;
            }
        },
        _ => panic!(),
    };
    x(&Box::new(0)); // shouldn't error.
}

fn f<T: Copy>(y: &T) -> T {
    *y
}

fn g(y: &[u8]) -> u8 {
    y[0]
}

trait Trait {}

impl<'a> Trait for &'a str {}

fn h(_: &dyn Trait) {}
#[warn(clippy::needless_borrow)]
#[allow(dead_code)]
fn issue_1432() {
    let mut v = Vec::<String>::new();
    let _ = v.iter_mut().filter(|&ref a| a.is_empty());
    let _ = v.iter().filter(|&ref a| a.is_empty());

    let _ = v.iter().filter(|&a| a.is_empty());
}

#[allow(dead_code)]
#[warn(clippy::needless_borrow)]
#[derive(Debug)]
enum Foo<'a> {
    Str(&'a str),
}
