#![allow(unused)]

pub(crate) fn raw_pointers() {
    let a = 42;
    //let m = &a as *const i32 as usize;
    let m = &a as *const i32;
    println!("a is here: {:?}", m);
    println!("&a: {:p}", &a);
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
        value: i32,
    }

    let f = Foo { value: 42111 };
    let ref_ref_ref_f = &&&f;

    println!("{}", ref_ref_ref_f.value);
    println!("{}", (***ref_ref_ref_f).value);
}

pub(crate) fn smart_pointers() {
    use std::ops::Deref;
    struct TT<T> {
        value: T,
    }

    impl<T> Deref for TT<T> {
        type Target = T;
        fn deref(&self) -> &T {
            println!("{} was used!", std::any::type_name::<T>());
            &self.value
        }
    }

    let foo = TT { value: "sec msg" };

    println!("{}", foo.len());

    let bar = TT { value: 1234 };

    println!("{}", bar.value);
}

pub(crate) fn smart_unsafe_code() {
    let a: [u8; 4] = [86, 14, 73, 64];
    let pointer_a = &a as *const u8 as usize;
    println!("pointer_a: {}", pointer_a);
    //let pointer_a = &a as *const u8;
    //println!("pointer_a: {:?}", pointer_a);

    let pointer_b = pointer_a as *const f32;
    println!("pointer_b: {:?}", pointer_b);

    let b = unsafe { *pointer_b };
    println!("b: {}", b);
}

pub(crate) fn heep_allocated_memory() {
    struct Pie;

    impl Pie {
        fn eat(&self) {
            println!("heep_allocated_memory");
        }
    }

    let heap_pie_box = Box::new(Pie);
    let heap_pie = Pie;

    println!(
        "&heap_pie: {:p}, &heap_pie_box: {:p}",
        &heap_pie, &heap_pie_box
    );

    heap_pie_box.eat();
    heap_pie.eat();
}

use std::error::Error;
pub(crate) fn failable_main_revisited() -> Result<(), Box<dyn Error>> {
    use std::fmt::Display;

    struct Pie;

    #[derive(Debug)]
    struct NotFreshError;

    impl Display for NotFreshError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "This pie is not fresh!")
        }
    }

    impl Error for NotFreshError {}

    impl Pie {
        fn eat(&self) -> Result<(), Box<dyn Error>> {
            Err(Box::new(NotFreshError))
        }
    }

    let heap_pie = Box::new(Pie);
    heap_pie.eat()?;
    Ok(())
}

pub(crate) fn reference_counting() {
    use std::rc::Rc;

    #[derive(Debug)]
    struct Pie;

    impl Pie {
        fn eat(&self) {
            println!("tastes better on the heap!")
        }
    }

    let heap_pie = Rc::new(Pie);
    let heap_pie2 = heap_pie.clone();
    let heap_pie3 = heap_pie2.clone();

    println!("1: {:p} -> {:p} -> {:?}", &heap_pie, &*heap_pie, *heap_pie);
    println!(
        "2: {:p} -> {:p} -> {:?}",
        &heap_pie2, &*heap_pie2, *heap_pie2
    );
    println!(
        "3: {:p} -> {:p} -> {:?}",
        &heap_pie3, &*heap_pie3, *heap_pie3
    );

    heap_pie3.eat();
    heap_pie2.eat();
    heap_pie.eat();

    println!("1: {:p} -> {:p} -> {:?}", &heap_pie, &*heap_pie, *heap_pie);
    println!(
        "2: {:p} -> {:p} -> {:?}",
        &heap_pie2, &*heap_pie2, *heap_pie2
    );
    println!(
        "3: {:p} -> {:p} -> {:?}",
        &heap_pie3, &*heap_pie3, *heap_pie3
    );
}

pub(crate) fn sharing_access() {
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Pie {
        slices: u8,
    }

    impl Pie {
        fn eat(&mut self) {
            //print!("sharing_access: {}", self.slices);
            self.slices -= 1;
            //println!(" -> {}", self.slices);
        }
    }

    let pie_cell = RefCell::new(Pie { slices: 8 });
    println!(
        "{:p} -> {:p} -> {:?}, {:?}",
        &pie_cell,
        &*pie_cell.borrow(),
        *pie_cell.borrow(),
        pie_cell
    );

    {
        let mut mut_ref_pie = pie_cell.borrow_mut();
        println!(
            "{:p} -> {:p} -> {:?}",
            &mut_ref_pie, &*mut_ref_pie, mut_ref_pie
        );
        mut_ref_pie.eat();
        println!(
            "{:p} -> {:p} -> {:?}",
            &mut_ref_pie, &*mut_ref_pie, mut_ref_pie
        );
        mut_ref_pie.eat();
        println!(
            "{:p} -> {:p} -> {:?}",
            &mut_ref_pie, &*mut_ref_pie, mut_ref_pie
        );
    }

    println!(
        "{:p} -> {:p} -> {:?}, {:?}",
        &pie_cell,
        &*pie_cell.borrow(),
        *pie_cell.borrow(),
        pie_cell
    );

    let ref_pie = pie_cell.borrow();
    println!("{:p} -> {:p} -> {:?}", &ref_pie, &*ref_pie, ref_pie);

    // The output for pie_cell is wrapped with RefCell {}, while the those for mut_ref_pie and
    // ref_pie are not.
    //
    // This is due to how Debug formatting works for different types:
    //  1. When printing `pie_cell`, you're seeing the Debug implementation for RefCell itself,
    //     which shows its internal structure including the wrapped value
    //  2. For `mut_ref_pie` and `ref_pie`, you're seeing the Debug implementation for RefMut
    //     and Ref respectively, which dereference to show the contained value directly
    //
    // In other words, `RefCell` shows its container structure, while `Ref` and `RefMut` act
    // more like transparent references to the inner value.
    //
    // Example output:
    // 0x7ffcb60bf090 -> 0x7ffcb60bf098 -> Pie { slices: 8 }, RefCell { value: Pie { slices: 8 } }
    // 0x7ffcb60bf180 -> 0x7ffcb60bf098 -> Pie { slices: 8 }
    // 0x7ffcb60bf180 -> 0x7ffcb60bf098 -> Pie { slices: 7 }
    // 0x7ffcb60bf180 -> 0x7ffcb60bf098 -> Pie { slices: 6 }
    // 0x7ffcb60bf090 -> 0x7ffcb60bf098 -> Pie { slices: 6 }, RefCell { value: Pie { slices: 6 } }
    // 0x7ffcb60bf450 -> 0x7ffcb60bf098 -> Pie { slices: 6 }
    //
    // Stack:                                  Heap:
    //  0x7ffcb60bf090  +----------------+      0x7ffcb60bf098  +-------------+
    //  (pie_cell)      | RefCell struct |----->|  Pie struct   |
    //                  +----------------+      |  slices: 6    |
    //                                          +-------------+
    //                                                 ^
    //    0x7ffcb60bf180  +----------------+           |
    //    (mut_ref_pie)   | RefMut struct  |------------
    //                    +----------------+
    //                                                 ^
    //    0x7ffcb60bf450  +----------------+           |
    //    (ref_pie)       | Ref struct     |------------
    //                    +----------------+
    //
}

pub(crate) fn sharing_across_threads() {
    use std::sync::Mutex;
    
    struct Pie;
    impl Pie {
        fn eat(&self) {
            println!("sharing_across_threads");
        }
    }

    // Create a Mutex that provides thread-safe access to Pie
    let mutex_pie = Mutex::new(Pie);

    // lock() returns MutexGuard, which is immediately used and dropped
    mutex_pie.lock().unwrap().eat();

    {
        // MutexGuard is stored, keeping the lock until the block ends
        let guard = mutex_pie.lock().unwrap();
        guard.eat();
    } // guard is dropped here, releasing the lock

    // These two calls each:
    // 1. Create a new MutexGuard (acquire lock)
    // 2. Call eat()
    // 3. Drop MutexGuard (release lock)
    mutex_pie.lock().unwrap().eat();
    mutex_pie.lock().unwrap().eat();

/*
Key concepts:
- Mutex<T>: Container providing mutual exclusion, owns the data of type T
- MutexGuard<T>: Temporary access token that:
  - Is returned by lock()
  - Provides access to &T or &mut T
  - Automatically releases lock when dropped
  - Ensures only one thread can access data at a time
*/

}

pub(crate) fn combining_smart_pointers() {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Pie {
        slices: u8,
    }

    impl Pie {
        fn eat_slice(&mut self, name: &str) {
            println!("{} took a slice", name);
            self.slices -= 1;
        }
    }

    struct SeaCreature {
        name: String,
        pie: Rc<RefCell<Pie>>,
    }

    impl SeaCreature {
        fn eat(&self){
            let mut p = self.pie.borrow_mut();  // Gets exclusive access to Pie
            p.eat_slice(&self.name);            //
        }                                       // RefMut dropped, releases access
    }

    let pie = Rc::new(RefCell::new(Pie {slices: 8})); // Rc count = 1
    println!("{:p}, {:?}", &pie, pie);
    println!("{:p}, {:?}", &*pie.borrow(), *pie.borrow());

    let ferris = SeaCreature {
        name: String::from("ferris"),
        pie: pie.clone(), // Rc count = 2
    };

    let sarah = SeaCreature {
        name: String::from("sarah"),
        pie: pie.clone(), // Rc count = 3
    };

    ferris.eat();
    println!("{:p}, {:?}", &pie, pie);
    println!("{:p}, {:?}", &pie.borrow(), *pie.borrow());
    println!("{:p}, {:?}", &*pie.borrow(), *pie.borrow());
    println!("{:p}, {:?}", &ferris.pie.borrow(), ferris.pie.borrow());
    println!("{:p}, {:?}", &*ferris.pie.borrow(), ferris.pie.borrow());
    sarah.eat();

    let p = pie.borrow();
    println!("{:?}, {}", p, p.slices);

/*
 * https://claude.ai/chat/667c3d7f-c3fd-4d8b-96a5-7331e148070e
 *
Let me create a more detailed diagram based on actual memory addresses and the code flow:

```
Stack:                      Heap:                            Data:

pie  +-------------+       +---------------+    +-----------------+
     | Rc pointer  |------>| Rc struct     |    | RefCell         |
     | 0xAAAA      |       | count: 3      |--->| +-------------+ |
     +-------------+       | data ptr      |    | | Pie         | |
                           +---------------+    | | slices: 8   | |
                                                | +-------------+ |
                                                +-----------------+
ferris  +-----------------+                          ^
        | SeaCreature     |                          |
        | name: "ferris"  |                          |
        | pie: Rc pointer |--------------------------+
        +-----------------+                          |
                                                     |
sarah   +-----------------+                          |
        | SeaCreature     |                          |
        | name: "sarah"   |                          |
        | pie: Rc pointer |--------------------------+
        +-----------------+

When ferris.eat() or sarah.eat() is called:
1. borrow_mut() creates temporary RefMut access to Pie
2. slices is modified
3. RefMut is dropped, releasing access

Operations:
- pie.clone() -> creates new Rc pointer to same data
- pie.borrow() -> gets Ref to Pie data
- pie.borrow_mut() -> gets RefMut to Pie data
```
*/
}

