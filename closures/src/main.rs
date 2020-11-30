use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<'a, T, U: 'a>
where
    T: Fn(U) -> &'a U,
{
    calculation: T,
    values: HashMap<U, U>,
}

impl<'a, T, U: 'a> Cacher<'a, T, U>
where
    T: Fn(U) -> &'a U,
    U: Eq + Hash + Clone,
{
    fn new(calculation: T) -> Cacher<'a, T, U> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> &U {
        let result = self.values.get(&arg);
        match result {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg.clone(), v);
                return v;
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        &num
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
    let simulated_user_specified_value = 100;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a: i32| &a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, &2);
}
