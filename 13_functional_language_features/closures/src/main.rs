//example situation : generate workout plan(calculation some data)
//we want to call the algorithm only when we need to
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

//avoid calling function more than once
//we can store closure result in struct
//To store it in struct, we need to specify the type of closure
//Each closures has itw own uniqure type even if two closures have the same signature
struct Cacher<T, K, V>
where
    T: Fn(K) -> V, //Generic K input and generic V is output
{
    calculation: T,
    value: HashMap<K, Option<V>>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: std::cmp::Eq + std::hash::Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        //create new instance and store closure in calculation as an arugment.
        Cacher {
            calculation,
            //because we haven't execute closure yet
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.value.get(&arg) {
            Some(v) => v.unwrap(), //return v as V
            None => {
                let v = (self.calculation)(arg); //arg is argument as K
                self.value.insert(arg, Some(v));
                v //return v as V
            }
        }
    }
}

fn main() {
    //intensity control - in real world , you might get this data from fronend interface
    let simulated_user_specified_value = 30;
    //generate workout plan - this can be taken by rand library.
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, rand_number: u32) {
    //what if it is the case high intensity and rand_number is 3
    //In this case, we don't need to call calculation.
    //So Here, To call calculation only once when we need it, we will use closure
    // we can set closure paremter lik |param1, param2|. It is |num| in below example
    //at this poing, we only define anonymous function
    let mut expensive_result = Cacher::new(|num| {
        //closure body start
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num //will be returned
    });
    //low intensity
    if intensity < 25 {
        //execute closure
        println!("Today, do {} pushups", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }
    //high intensity
    else {
        if rand_number == 3 {
            //nothing happened with closure
            println!("Take a break today! remember to stay hydrated");
        } else {
            //execute closure
            println!(
                "Today, run for {}minutes",
                expensive_result.value(intensity)
            );
        }
    }
}

//Closures can capture values from their environment in three ways
//1. (take ownership) FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment.
//   To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
//2. (mutably borrowing) FnMut can change the environment because it mutably borrows values.
//3. (immutably borrowing) Fn borrows values from the environment immutably.

fn _example_of_closure() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x; //closure take ownership of x

    //println!("can't use x here: {:?}", x); -> this line cause error because x already borrowed by closure

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
