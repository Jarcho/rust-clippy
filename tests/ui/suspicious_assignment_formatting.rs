//@no-rustfix
//@aux-build:proc_macros.rs
#![deny(clippy::suspicious_assignment_formatting)]

use proc_macros::{external, inline_macros, with_span};

#[rustfmt::skip]
#[inline_macros]
fn main() {
    {
        let mut x = 42;
        let y = &45;

        x =- 35; //~ suspicious_assignment_formatting
        x =* y; //~ suspicious_assignment_formatting

        x=-32;
        x =-32;
        x =(-32);
        x =-/* comment */32;
    }

    {
        let mut x = false;
        let y = false;
        x =! y; //~ suspicious_assignment_formatting
    }

    with_span! {
        span
        let mut x = 42;
        x =- 35;
    }

    external! {
        let mut x = 42;
        x =- 35;
    }

    inline! {
        let mut x = 42;
        x =- 35; //~ suspicious_assignment_formatting
    }

    {
        let mut x = 42;
        inline! {
            $x =- $35; //~ suspicious_assignment_formatting
        }
    }

    {
        macro_rules! m {
            ($($t:tt)*) => {
                let mut x = 42;
                x $($t)* 35;
            }
        }
        m!(=-);
    }

    {
        macro_rules! m {
            ($($t:tt)*) => {
                let mut x = 42;
                x $($t)*;
            }
        }
        m!(=- 35);
    }
}
