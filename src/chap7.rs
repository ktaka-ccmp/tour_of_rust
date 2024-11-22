#![allow(unused)]

pub(crate) fn encapsulation_with_methods() {
    struct SeaCreature {
        noise: String,
    }

    impl SeaCreature {
        fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    let creature = SeaCreature {
        noise: String::from("bulb"),
    };
    println!("{}", creature.get_sound());
}

pub(crate) fn abstraction_with_selective_exposure() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }

    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("bulb"),
    };
    println!("{}", creature.get_sound());
}

pub(crate) fn polymorphism_with_traits() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }

    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    trait NoiseMaker {
        fn make_noise(&self);
    }

    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }

    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("bulb"),
    };

    creature.make_noise();
}

pub(crate) fn implemented_methods_on_traits() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }

    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    trait NoiseMaker {
        fn make_noise(&self);

        fn make_a_lot_of_noise(&self) {
            self.make_noise();
            self.make_noise();
            self.make_noise();
        }
    }

    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }

    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("bulb"),
    };

    creature.make_a_lot_of_noise();
}

pub(crate) fn trait_inheritance() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }

    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    trait NoiseMaker {
        fn make_noise(&self);
    }

    trait LoudNoiseMaker: NoiseMaker {
        fn make_a_lot_of_noise(&self) {
            self.make_noise();
            self.make_noise();
            self.make_noise();
        }
    }

    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }

    impl LoudNoiseMaker for SeaCreature {}

    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub blurb"),
    };

    creature.make_a_lot_of_noise();
}

pub(crate) fn dynamic_vs_static_dispatch() {
    struct SeaCreature {
        name: String,
        noise: String,
    }

    impl SeaCreature {
        fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    trait NoiseMaker {
        fn make_noise(&self);
    }

    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }

    fn static_make_noise(creature: &SeaCreature) {
        creature.make_noise();
    }

    fn dynamic_make_noise(nm: &dyn NoiseMaker) {
        nm.make_noise();
    }

    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };

    static_make_noise(&creature);
    dynamic_make_noise(&creature);
}

pub(crate) fn generic_functions() {
    struct SeaCreature {
        name: String,
        noise: String,
    }

    impl SeaCreature {
        fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    trait NoiseMaker {
        fn make_noise(&self);
    }

    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }

    struct LandCreature {
        name: String,
        noise: String,
    }

    impl NoiseMaker for LandCreature {
        fn make_noise(&self) {
            println!("{}", &self.noise);
        }
    }

    fn generic_make_noise<T>(creature: &T)
    where
        T: NoiseMaker,
    {
        creature.make_noise();
    }

    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("bulb"),
    };

    generic_make_noise(&creature);

    let creature = LandCreature {
        name: String::from("Cow"),
        noise: String::from("Mowoooo"),
    };

    generic_make_noise(&creature);
}

pub(crate) fn generic_function_shorthand() {
    struct SeaCreature {
        name: String,
        noise: String,
    }

    impl SeaCreature {
        fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    trait NoiseMaker {
        fn make_noise(&self);
    }

    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }

    struct LandCreature {
        name: String,
        noise: String,
    }

    impl NoiseMaker for LandCreature {
        fn make_noise(&self) {
            println!("{}", &self.noise);
        }
    }

    fn generic_make_noise(creature: &impl NoiseMaker) {
        creature.make_noise();
    }

    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("bulb"),
    };

    generic_make_noise(&creature);

    let creature = LandCreature {
        name: String::from("Cow"),
        noise: String::from("Mowoooo"),
    };

    generic_make_noise(&creature);
}

pub(crate) fn box_() {
    struct SeaCreature {
        name: String,
        noise: String,
    }

    impl SeaCreature {
        fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    trait NoiseMaker {
        fn make_noise(&self);
    }

    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }

    struct Ocean {
        animals: Vec<Box<dyn NoiseMaker>>,
    }

    let ferris = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    let sarah  = SeaCreature {
        name: String::from("Sarah"),
        noise: String::from("swish"),
    };

    let ocean = Ocean {
        animals: vec![Box::new(ferris), Box::new(sarah)],
    };

    for a in ocean.animals.iter() {
       a.make_noise();
    }
}

 




 




 




 




