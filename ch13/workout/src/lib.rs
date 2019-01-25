use std::thread;
use std::time::Duration;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulation_returns_given() {
        assert_eq!(8, simulated_expensive_calculation(8));
    }

    #[test]
    fn call_cahcer_with_different_values() {
        let mut k = Cacher::new(|a| a);

        let v1 = k.value(1);
        let v2 = k.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 1);
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_millis(2));
    intensity
}

#[derive(Debug)]
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calulation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(func: T) -> Cacher<T> {
        Cacher {
            calulation: func,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calulation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_millis(2));
        intensity
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
