#![allow(unused)]

pub(crate) fn ownership() {
    struct Foo {
        x: i32,
    }

    let foo = Foo { x: 42 };
    println!("{}", foo.x);
}

pub(crate) fn scope_based_resource_management() {
    struct Foo {
        x: i32,
    }

    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
}

pub(crate) fn dropping_is_hierarchical() {
    struct Bar {
        x: i32,
    }

    struct Foo {
        bar: Bar,
    }

    let foo = Foo { bar: Bar { x: 43 } };
    println!("{}", foo.bar.x);
}

pub(crate) fn moving_ownership() {
    struct Foo {
        x: i32,
    }

    fn do_something(f: Foo) {
        println!("{}", f.x);
    }

    let foo = Foo { x: 44 };
    do_something(foo);
}

pub(crate) fn returning_ownership() {
    struct Foo {
        x: i32,
    }

    fn do_something() -> Foo {
        Foo { x: 45 }
    }

    let foo = do_something();
    println!("{}", foo.x);
}

pub(crate) fn borrowing_ownership_with_references() {
    struct Foo {
        x: i32,
    }

    let foo = Foo { x: 44 };
    let f = &foo;
    println!("{}", f.x);
}

pub(crate) fn borrowing_mutable_ownership_with_references() {
    #[derive(Clone)]
    struct Foo {
        x: i32,
    }

    fn do_something(f: Foo) {
        println!("{}", f.x);
    }

    let mut foo = Foo { x: 44 };
    do_something(foo.clone());

    let f = &mut foo;

    //do_something(foo);

    f.x = 13;

    println!("{}", foo.x);

    foo.x = 7;
    do_something(foo);
}

pub(crate) fn dereferencing() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f;
    *f = 13;
    //let bar = *f;
    println!("{}", foo);
    println!("{}", bar);
}

pub(crate) fn passing_around_borrowed_data() {
    struct Foo {
        x: i32,
    }

    fn do_something(f: &mut Foo) {
        f.x += 1;
        println!("f.x: {}", f.x);
    }

    let mut foo = Foo { x: 46 };
    println!("foo.x: {}", foo.x);
    do_something(&mut foo);
    println!("foo.x: {}", foo.x);

    do_something(&mut foo);
    println!("foo.x: {}", foo.x);
}

pub(crate) fn references_of_references() {
    struct Foo {
        x: i32,
    }

    fn do_something(a: &Foo) -> &i32 {
        &a.x
    }

    let mut foo = Foo { x: 46 };
    let x = &mut foo.x;
    *x = 13;
    println!("foo.x: {}", foo.x);

    let y = do_something(&foo);
    println!("y: {}", y);
}

pub(crate) fn explicit_lifetimes() {
    struct Foo {
        x: i32,
    }

    fn do_something0(foo: &Foo) -> &i32 {
        &foo.x
    }

    fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
        &foo.x
    }

    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 12;

    //let y = do_something(&foo);
    let y = do_something0(&foo);
    println!("y: {}", y);
}

pub(crate) fn multiple_lifetimes() {
    struct Foo {
        x: i32,
    }

    fn do_something0(foo: &Foo) -> &i32 {
        &foo.x
    }

    fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
        println!("foo_a.x: {}", foo_a.x);
        println!("foo_b.x: {}", foo_b.x);
        &foo_b.x
    }

    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };
    let x = do_something(&foo_a, &foo_b);
    println!("x: {}", x);
    println!("foo_a.x: {}", foo_a.x);
}

pub(crate) fn static_lifetimes() {
    static PI: f64 = 3.1415999999;

    static mut SECRET: &'static str = "swordfish";
    //static SECRET: &'static str = "swordfish";

    let msg: &'static str = "Hello World";
    let p: &'static f64 = &PI;
    println!("msg: {}, p: {}", msg, p);

    //println!("SECRET: {}", SECRET);
    unsafe {
        println!("SECRET: {}", SECRET);
        SECRET = "abcdefg";
        println!("SECRET: {}", SECRET);
    }
}

pub(crate) fn lifetime_in_data_types() {
    struct Foo<'a> {
        i: &'a i32,
    }

    struct Bar<'a> {
        i: &'a i32,
    }

    let x = 42;
    let foo = Foo { i: &x };

    let bar = Bar { i: &x };

    println!("foo.i: {}", foo.i);
    println!("bar.i: {}", bar.i);
}
