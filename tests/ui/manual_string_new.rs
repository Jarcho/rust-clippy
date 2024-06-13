#![warn(clippy::manual_string_new)]
#![allow(clippy::unnecessary_fallible_conversions)]

macro_rules! create_strings_from_macro {
    // When inside a macro, nothing should warn to prevent false positives.
    ($some_str:expr) => {
        let _: String = $some_str.into();
        let _ = $some_str.to_string();
    };
}

fn main() {
    // Method calls
    let _ = "".to_string();
    let _ = "no warning".to_string();

    let _ = "".to_owned();
    let _ = "no warning".to_owned();

    let _: String = "".into();
    let _: String = "no warning".into();

    let _: SomeOtherStruct = "no warning".into();
    let _: SomeOtherStruct = "".into(); // No warning too. We are not converting into String.

    // Calls
    let _ = String::from("");
    let _ = <String>::from("");
    let _ = String::from("no warning");
    let _ = SomeOtherStruct::from("no warning");
    let _ = SomeOtherStruct::from(""); // Again: no warning.

    let _: String = From::from("");
    let _: String = From::from("no warning");
    let _: SomeOtherStruct = From::from("no warning");
    let _: SomeOtherStruct = From::from(""); // Again: no warning.

    // Macros (never warn)
    create_strings_from_macro!("");
    create_strings_from_macro!("Hey");
}

struct SomeOtherStruct {}

impl From<&str> for SomeOtherStruct {
    fn from(_value: &str) -> Self {
        Self {}
    }
}
