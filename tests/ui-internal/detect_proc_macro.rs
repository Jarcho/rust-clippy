#![warn(clippy::detect_proc_macros)]
#![allow(irrefutable_let_patterns, missing_abi, while_true, clippy::all, clippy::pedantic)]

#[derive(Clone, Copy)] //~ detect_proc_macros
struct S;
impl S {
    fn f0(self) {}
    fn f1<A>(self, _: A) {}
    fn f2<A, B>(self, _: A, _: B) {}
    fn f3<A, B, C>(self, _: A, _: B, _: C) {}
}

fn main() {
    {
        true;
        false;
        1;
        1_i8;
        1_i16;
        1_i32;
        1_i64;
        1_i128;
        1_isize;
        1_u8;
        1_u16;
        1_u32;
        1_u64;
        1_u128;
        1_usize;
        1_f32;
        1_f64;
        1.0;
        1.0_f32;
        1.0_f64;
    }
    {
        let _: [(); _] = [];
        let _: [(); _] = ([]);
        [()];
        ([(())]);
        ([(())]);
        ([(()), ()]);
    }
    {
        &1;
        &mut 1;
        (&1);
        (&(1));
        let mut x = 1;
        &raw mut x;
        &raw const x;
    }
    {
        // _ = 1;
        // (_) = 1;
        // _ = (1);
        let mut x = 0;
        x = 1;
        (x) = (1);
        // (x,) = (1,);
        let mut y = 0;
        // ((x), (y),) = (1, 1,);
    }
    {
        let mut x = 0i32;
        x += 1;
        (x) -= 2;
        (x) *= (3);
        x &= 1;
    }
    {
        1 + 1;
        (1) * 2;
        (false) || (true);
    }
    {
        {};
        {
            1
        };
        unsafe {};
        'l1: {};
        'l3: {
            break 'l3;
            (break 'l3);
            break 'l3 ();
            break 'l3 (());
            (break 'l3 (()));
        };
        async {};
    }
    {
        fn f0() {}
        fn f1<A>(_: A) {}
        fn f2<A, B>(_: A, _: B) {}
        fn f3<A, B, C>(_: A, _: B, _: C) {}

        f0();
        let f0 = f0;
        (f0)();
        ({ f0 })();
        f1(0);
        f1(0);
        ({ f1 })((0));
        f2(0, 0);
        f3((0), (0), (0));
    }
    {
        0 as u64;
        (1) as u32;
        (2) as (u16);
        &0 as *const (i32);
    }
    {
        || {};
        (|x| -> u32 { x })(0u32);
        (|x| x)(0u32);
        |x: u32| {};
        move |x: u32, y: u16| {};
        async || {};
        async |x: u32| {};
        async move |x: u32| {};
    }
    {
        const { 0 };
        (const {
            ();
            (())
        });
    }
    {
        let x = (0, 1, 2);
        x.0;
        (x).1;
        ({ x }.2);

        struct S {
            x: u32,
        }
        ((S { x: 0 }).x);
    }
    {
        if
        /* comment */
        true {
            0;
        }
        if true {
            0
        }
        /* comment */
        else {
            0
        };
        (if (true) {
            0;
        } else {
            0;
        });
        if let x = 5
            && let _ = 0
            && let (y) = x
        {}
    }
    {
        let x = [0; 5];
        x[0];
        (x)[0];
        ({ x }[(x[0])]);
    }
    {
        loop {
            break;
            break ();
            (break);
            (break (()));
            continue;
            (continue);
        }
        'l1: loop {
            break 'l1;
            (break 'l1);
            continue 'l1;
            (continue 'l1);
        }
        while (true) {
            break;
        }
        'l2: while let _x = true {
            break 'l2;
        }
        for _ in 0..0 {}
        ('l3: for (_x) in (0..0) {
            break 'l3;
        });
    }
    {
        (match (true) {
            x if x => {},
            _ => {},
        });
        || -> Option<u32> {
            Some(1)?;
            (Some(1))?;
            None
        };
        let x = async || -> u32 { 0 };
        let x = async move || -> u32 { x().await };
        let mut x = 0;
        #[rustfmt::skip]
        {
            match x {
                mut x if true => {},
                ref x if true => {},
                ref mut x if true => {},
                // mut ref x if true => {},
                // mut ref mut x if true => {},
                ref x @ (| (0) | (1)) if true => {},
                ((_)) => {},
            }
        };
        match &x {
            &(x) if true => {},
            (&x) if true => {},
            _ => {},
        }
        match &mut x {
            &mut x => {},
        }
    }
    {
        S.f0();
        ({ S }.f0());
        S.f1(0);
        ({ S }.f1((0)));
        (S).f2(0, 1);
        ((*&S).f3(1, 2, (3)));
    }
    {
        let _: () = core::default::Default::default();
        let _: () = ::core::default::Default::default();
        <u32 as core::default::Default>::default();
        <(u32) as ::core::default::Default>::default();
        (<(u32) as Default>::default());
        <u32>::default();
        <(u32)>::default();
    }
    {
        || -> u32 {
            (return (1));
        };
        (return);
    }
    {
        ();
        (());
        (());
        (1,);
        (1, 2);
        (1, 2);
        ((1, 2, 3));
        (((1), (2), (3)));
    }
    {
        let mut x = 1i32;
        -x;
        -(x);
        (-(x));
        !x;
        *(&x);
        *&mut (x);
        unsafe {
            (*&raw mut x);
            *&raw const x;
        }
    }
    {
        ..;
        (..);
        ..1;
        ..(1);
        (..(1));
        1..;
        (1)..;
        ((1)..);
        1..2;
        (1)..2;
        (1)..=2;
        ..=1;
    }
    {
        const C1: () = ();
        pub const C2: () = ();
        pub(crate) const C3: (u32) = (0);
    }
    {
        enum E1 {
            V1,
            V2(),
            V3(()),
            V4((), u32, (u16)),
            V5 {},
            V6 { _s: (((()))) },
        }
        pub enum E2 {}
        enum E3<T> {
            V(T),
        }
        enum E4
        where
            Self: Sized, {}
        enum E5<T>
        where
            T: Sized,
        {
            V(T),
        }
    }
    {
        extern crate core;
        pub(crate) extern crate alloc;
        extern crate core as c1;
        pub extern crate core as c2;
    }
    {
        fn f1() {}
        pub fn f2() {}
        pub(crate) fn f3() {}
        const fn f4() {}
        unsafe fn f5() {}
        async fn f6() {}
        #[rustfmt::skip]
        extern fn f7() {}
        extern "Rust" fn f8() {}
        extern "C" fn f9() {}
        const unsafe extern "C" fn f10() {}
        const unsafe extern "Rust" fn f11() {}
        const unsafe extern "C" fn f12() {}
        async unsafe fn f13() {}
        pub async unsafe fn f14() {}
        async fn f16() -> (u32, u32) {
            (0, 0)
        }
    }
    {
        #[rustfmt::skip]
        unsafe extern {}
        unsafe extern "Rust" {}
        unsafe extern "C" {}
    }
    {
        struct S<T>(T);
        impl (S<u32>) {}
        impl<T: Iterator> S<T> {}
        impl S<i32> where i32: Sized {}

        trait T1 {}
        unsafe trait T2 {}
        impl T1 for S<u32> {}
        unsafe impl<T> T2 for S<T> {}
    }
    {
        macro_rules! m1 {
            () => {};
        }
    }
    {
        mod m1 {}
        pub mod m2 {}
        pub(crate) mod m3 {}
    }
    {
        static S1: () = ();
        pub static S2: () = ();
        pub(crate) static S3: (u32) = (0);
        static mut S4: i32 = 0;
    }
    {
        struct S1;
        pub struct S2;
        pub(crate) struct S3<const N: usize>;
        struct S4
        where
            u32: Sized;

        struct S5();
        struct S6<const N: usize>();
        struct S7(u32)
        where
            u32: Copy;
        pub(crate) struct S8(());

        struct S9 {
            f1: (u32),
            pub f2: u16,
            pub(crate) f3: (()),
        }
        struct S10<T> {
            f1: T,
        }
        struct S11
        where
            u16: Sized, {}
    }
    {
        trait T1 {
            fn f1();
            unsafe fn f2();
        }
        pub trait T2: Sized {}
        pub(crate) trait T3<T> {}
        pub(crate) trait T4<T: Fn((u32)) -> (u32), U>: Sized {}
        trait T5
        where
            i16: Sized,
        {
        }
        trait T6: Sized
        where
            u32: Sized,
        {
        }
    }
    {
        type T1 = u32;
        pub type T2 = u32;
        pub(crate) type T3 = (u32);
        type T4<'a, T> = (&'a T, T);
    }
}
