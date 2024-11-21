#![allow(unused)]

pub(crate) fn hello_world() {
    println!("Hello, world!");
}

pub(super) fn variables() {
    let x = 13;
    println!("{}", x);

    let x = 3.32746758;
    println!("{}", x);

    let x: f64 = 2.111;
    println!("{}", x);

    let x;
    x = "hello";
    println!("{}", x);
}

pub(super) fn changing_variables() {
    let mut x = 42;
    println!("{}", x);
    x = 3;
    println!("{}", x);
}

pub(super) fn basic_types() {
    let x = 12;
    let a = 12u8;
    let b = 4.3;
    let c = 4.4f32;
    let d = 'r';
    let ferris = "🦀";
    let bv = true;
    let t = (13, false);
    let sentence = "hello fucking world";

    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        x, a, b, c, d, ferris, bv, t.1, t.0, sentence
    );
}

pub(super) fn basic_type_conversion() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{} {}", t, t as u8);
}

const PI: f32 = 3.1415933333;

pub(super) fn constants() {
    println!("{}", PI);
}

pub(super) fn arrays() {
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}, {}", nums, nums[1]);
}

pub(super) fn functions() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn subtract(x: i32, y: i32) -> i32 {
        x - y
    }

    println!("42 + 12 = {}", add(42, 12));
    println!("42 - 12 = {}", subtract(42, 12));
}

pub(super) fn multiple_return_values() {
    fn swap(x: i32, y: i32) -> (i32, i32) {
        (y, x)
    }

    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}

pub(super) fn returning_nothing() {
    fn make_nothing() -> () {
        ()
    }
    fn make_nothing2() {}

    let a = make_nothing();
    let b = make_nothing2();

    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

pub(super) fn if_else() {
    let x = 41;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
}

pub(super) fn loop_() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 48 {
            break;
        }
    }
    println!("x: {}", x);
}

pub(super) fn while_() {
    let mut x = 0;
    while x != 52 {
        x += 1;
    }
    println!("x: {}", x);
}

pub(super) fn for_() {
    for x in 0..5 {
        println!("x: {}", x);
    }
    for x in 0..=5 {
        println!("x: {}", x);
    }
}

pub(super) fn match_() {
    let x = 42;
    match x {
        0 => {
            println!("found zero");
        }
        matched_num @ 10..=100 => {
            println!("found {}", matched_num);
        }
        _ => {
            println!("found somethingelse");
        }
    }
}

pub(super) fn returning_values_from_loop() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found 13";
        }
    };
    println!("We found: {}", v);
}

pub(super) fn returning_values_from_block_expressions() {
    fn example() -> i32 {
        let x = 42;
        let v = if x < 42 { -1 } else { 1 };
        println!("from if: {}", v);

        let food = "apple";
        let result = match food {
            "hotdog" => "is hotdog",
            a @ "apple" => &format!("is {}", a).to_string(),
            _ => "is inot hotdog",
        };
        println!("food: {}", result);

        let v = {
            let a = 1;
            let b = 2;
            a + b
        };
        println!("from block: {}", v);

        v + 4
    }

    println!("from function: {}", example());
}

pub(super) fn calling_methods() {
    let s = String::from("Hello world!");
    println!("{} has {} characters.", s, s.len());
}

pub(super) fn creating_data_in_memory() {
    struct SeaCreature {
        animal_type: String,
        name: String,
        arms: i32,
        legs: i32,
        weapon: String,
    }

    let ferris = SeaCreature {
        animal_type: String::from("crab"),
        name: String::from("ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("brain"),
    };

    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon.",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon.",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs, sarah.weapon
    );
}

pub(super) fn tuple_like_structs() {
    struct Location(i32, i32);

    let loc = Location(42, 32);
    println!("Location: {}, {}", loc.0, loc.1);
}

pub(super) fn enumerations() {
    #[derive(Debug)]
    enum Species {
        Crab,
        Octopus,
        Fish,
        Clam,
    }

    struct SeaCreature {
        species: Species,
        name: String,
        arms: i32,
        legs: i32,
        weapon: String,
    }

    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        //Species::Crab => println!("{} is a {:?}.", ferris.name, ferris.species),
        _ => println!("{} is a {:?}.", ferris.name, ferris.species),
    }
}

pub(super) fn enumerations_with_data() {
    enum Species {
        Crab,
        Octopus,
        Fish,
        Clam,
    }
    enum PoisonType {
        Acidic,
        Painful,
        Lethal,
    }
    enum Size {
        Big,
        Small,
    }
    enum Weapon {
        Claw(i32, Size),
        Poison(PoisonType),
        None,
    }

    struct SeaCreature {
        species: Species,
        name: String,
        arms: i32,
        legs: i32,
        weapon: Weapon,
    }

    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => match ferris.weapon {
            Weapon::Claw(num_claws, size) => {
                let size_description = match size {
                    Size::Big => "L-size",
                    Size::Small => "S-size",
                };
                println!(
                    "ferris is a crab with {} {} claws.",
                    num_claws, size_description
                );
            }
            _ => println!("ferris is a crab with some other weapon."),
        },
        _ => println!("some text"),
    }
}
