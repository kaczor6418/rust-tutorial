use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    values: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Hash + Clone,
{
    fn new(calculation: T) -> Cacher<T, U> {
        return Cacher {
            calculation,
            values: HashMap::new(),
        };
    }

    fn value(&mut self, arg: U) -> U {
        let value_ref = &mut self.values;
        let calculation_ref = &self.calculation;
        let result = value_ref.get(&arg);
        return match result {
            Some(v) => v.clone(),
            None => self
                .values
                .entry(arg.clone())
                .or_insert_with(|| (calculation_ref)(arg.clone()))
                .clone(),
        };
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
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

fn main() {
    let simulated_user_specified_value = 11;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a: i32| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
