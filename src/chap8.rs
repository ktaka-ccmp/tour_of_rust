#![allow(unused)]

pub(crate) fn raw_pointers() {
        let a = 42;
        let m = &a as *const i32 as usize;
        println!("a is here: {}", m);
}

pub(crate) fn the_asterisk_operator() {
    let a: i32 = 423;
    let ref_ref_ref_a: &&&i32 = &&&a;
    let ref_a: &i32 = **ref_ref_ref_a;
    let b: i32 = *ref_a;
    println!("a: {}, b: {}", a, b);
}

pub(crate) fn the_dot_operator() {
    struct Foo {
        value: i32
    }

    let f = Foo { value: 42111 };
    let ref_ref_ref_f = &&&f;

    println!("{}", ref_ref_ref_f.value);
    println!("{}", (***ref_ref_ref_f).value);
}

pub(crate) fn smart_pointers() {
    use std::ops::Deref;
    struct TT<T> {
        value: T
    }

    impl<T> Deref for TT<T> {
        type Target = T;
        fn deref(&self) -> &T {
            println!("{} was used!", std::any::type_name::<T>());
            &self.value
        }
    }

    let foo = TT {
        value: "sec msg"
    };

    println!("{}", foo.len());

    let bar = TT {
        value: 1234
    };

    println!("{}", bar.value);

}



