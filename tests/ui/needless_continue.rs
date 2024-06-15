//@aux-build:proc_macros.rs

#![warn(clippy::needless_continue)]
#![allow(while_true, clippy::empty_loop)]
use proc_macros::{external, inline_macros, with_span};

#[inline_macros]
fn main() {
    loop {
        continue; //~ needless_continue
    }

    while true {
        let _ = 1 + 1;
        continue; //~ needless_continue
    }

    for _ in 0..1 {
        let _ = match 0 {
            0 => 0,
            1 => 1,
            _ => 2,
        };
        continue; //~ needless_continue
    }

    loop {
        if true {
            continue; //~ needless_continue
        }
    }

    loop {
        if true {
            continue; //~ needless_continue
        } else {
            println!();
        }
    }

    loop {
        if true {
            println!();
        } else {
            continue; //~ needless_continue
        }
    }

    loop {
        match 0 {
            0 => break,
            1 => continue, //~ needless_continue
            _ => {},
        }
    }

    loop {
        if true {
            continue;
        }
        match 0 {
            0 => match 0 {
                0 => continue, //~ needless_continue
                1 => {
                    if true {
                        continue; //~ needless_continue
                    } else {
                        break;
                    }
                },
                _ => {},
            },
            1 => {
                let _ = 1;
                continue; //~ needless_continue
            },
            _ => break,
        }
    }

    'outer: loop {
        'inner: loop {
            match 0 {
                0 => continue 'outer,
                1 => continue 'inner, //~ needless_continue
                _ => continue,        //~ needless_continue
            }
        }
    }

    'outer: loop {
        loop {
            match 0 {
                0 => continue 'outer,
                1 => continue, //~ needless_continue
                _ => break,
            }
        }
    }

    external! {
        loop {
            continue;
        }
    }

    with_span! {
        span
        loop {
            continue;
        }
    }

    inline! {
        loop {
            continue; //~ needless_continue
        }

        loop $(@block {
            continue;
        })

        loop {
            $(@stmt continue)
        }

        loop {
            $(@expr continue);
        }

        loop {
            if true $(@block {
                continue;
            })
        }

        loop {
            if true {
                $(@stmt continue)
            } else {
                continue; //~ needless_continue
            }
        }

        loop {
            $(@stmt match 0 {
                0 => continue,
                1 => break,
                _ => {},
            })
        }
    }
}
