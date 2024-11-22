#![allow(unused)]

pub(crate) fn string_literals() {
    //let a: &'static str = "hi 🦀";
    let a: &str = "hi 🦀";
    println!("{}, length: {}", a, a.len());
}

pub(crate) fn raw_string_literals() {
    //let a: &'static str = r#"
    let a: &str = r#"
        <div class="advice">
            Raw strings are useful for some situations.
        </div>
        "#;
    println!("{}", a);
}

pub(crate) fn string_slice() {
    let a = "hi 🦀";
    println!("{}", a.len());

    let first_word = &a[0..2];
    let second_word = &a[3..7];

    println!("{}, {}", first_word, second_word);
}

pub(crate) fn chars() {
    let a = "hi 🦀";
    let chars = a.chars().collect::<Vec<char>>();
    println!("a.len: {}", a.len());
    println!("chars.len: {}", chars.len());

    println!("chars[3]: {}", chars[3]);
    println!("as u32: {}", chars[3] as u32);
}

pub(crate) fn string() {
    let mut hello = String::from("hello");
    hello.push_str(" world");
    hello = hello + "!";
    println!("{}", hello);
}

pub(crate) fn text_as_function_parameters() {
    fn say(msg: &str) {
        println!("{}!!!", msg.to_uppercase());
    }

    say("hello");
    say(&String::from("good bye"));
}

pub(crate) fn building_strings() {
    let hello = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", hello);
    println!("{}", abc);
}

pub(crate) fn formating_string() {
    let a = 42;
    let f = format!("a: {}", a);
    println!("{}", f);
}

pub(crate) fn converting_strings() -> Result<(), std::num::ParseIntError> {
    let a = 22;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a, b);
    Ok(())
}
