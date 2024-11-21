#![allow(unused)]

pub(crate) fn what_are_generic_types() {
    struct BagOfHolding<T> {
        item: T,
    }

    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: true };
    let float_bag = BagOfHolding { item: 2.14 };

    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "apple" },
    };

    println!(
        "{} {} {} {}",
        i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
    );
}

pub(crate) fn option() {
    struct BagOfHolding<T> {
        item: Option<T>,
    }

    let i32_bag = BagOfHolding::<i32> { item: None };
    if i32_bag.item.is_none() {
        println!("Nothing in the bag.");
    } else {
        println!("There is {} in the bag.", i32_bag.item.unwrap());
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };
    if i32_bag.item.is_some() {
        println!("There is {} in the bag.", i32_bag.item.unwrap());
    } else {
        println!("Nothing in the bag.");
    }
}

pub(crate) fn result() {
    fn do_something(i: i32) -> Result<f32, String> {
        if i == 42 {
            Ok(13.01)
        } else {
            Err(String::from("Wrong number"))
        }
    }

    let result = do_something(12);
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}", e),
    }

    let result = do_something(42);
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}", e),
    }
}

pub(crate) fn gracefull_error_handling() -> Result<(), String> {
    fn do_something(i: i32) -> Result<f32, String> {
        if i == 42 {
            Ok(13.01)
        } else {
            Err(String::from("Wrong number"))
        }
    }

    let v = do_something(42)?;
    println!("found {}", v);
    Ok(())
}

pub(crate) fn ugly_option_result_handling() -> Result<(), String> {
    fn do_something(i: i32) -> Result<f32, String> {
        if i == 42 {
            Ok(13.01)
        } else {
            Err(String::from("Wrong number"))
        }
    }

    let v = do_something(42).unwrap();
    println!("found {}", v);

    let v = do_something(1).unwrap();
    println!("found {}", v);
    Ok(())
}

pub(crate) fn vectors() {
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);
    i32_vec.push(4);

    let mut float_vec = Vec::new();
    float_vec.push(1.1);
    float_vec.push(2.1);
    float_vec.push(3.1);
    float_vec.push(4f32);

    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in i32_vec.iter() {
        println!("{}", word);
    }
    for word in float_vec.iter() {
        println!("{}", word);
    }
    for word in string_vec.iter() {
        println!("{}", word);
    }
}
