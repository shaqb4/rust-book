use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

pub struct Cacher<T, U: Eq + Hash + Copy, V: Clone>
where 
    T: Fn(U) -> V,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U: Eq + Hash + Copy, V: Clone> Cacher<T, U, V> 
where 
    T: Fn(U) -> V,
{
    pub fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: U) -> &V {
        self.values.entry(arg).or_insert((self.calculation)(arg))
        /*let val = self.values.get(arg).clone();
        match val {
            Some(&v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(*arg, v);
                v.clone()
            }
        }*/
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(&intensity));
        }
    }
}

pub fn move_closures() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    //println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}