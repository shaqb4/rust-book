fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("");

    let b = [10, 20, 30, 40, 50];
    for element in b.iter() {
        println!("the value is: {}", element);
    }


    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");


    println!("12 F = {} C", farenheit_to_celsius(12.0));
    println!("-11.11 F = {} F", celsius_to_farenheit(-11.11));


    for i in 1..=13 {
        println!("fib({}) = {}", i, nth_fib(i));
    }
}

fn celsius_to_farenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

fn farenheit_to_celsius(farenheit: f64) -> f64 {
    (farenheit - 32.0) * 5.0 / 9.0
}

fn nth_fib(n: u32) -> u32 {
    let mut prev = 0;
    let mut num = 1;
    let mut tmp = num;

    for _ in 1..n {
        num = num + prev;
        prev = tmp;
        tmp = num;
    }

    num
}