use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;
use std::cmp::Eq;
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
#[derive(Debug)]
struct Cacher<T, U, V>
    where 
    U: Hash + Eq + Clone,
    V: Clone,
    T: Fn(U) -> V,
{
    calculation: T,
    values: HashMap<U, V>,
}
impl<T, U, V> Cacher<T, U, V>
    where 
    U: Hash + Eq + Clone,
    V: Clone,
    T: Fn(U) -> V,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V{
        match self.values.entry(arg.clone()) {
        	Occupied(occupied_entry) => {
        		let value = occupied_entry.get();
        		value.clone()
        	}
        	Vacant(vacant_entry) => {
        		let value = (self.calculation)(arg);
        		vacant_entry.insert(value.clone());
        		value
        	}
        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value("yeah");
    let v2 = c.value("no");
    assert_eq!(v2, "no");
}