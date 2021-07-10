#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

struct Color(i32, i32, i32);

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Found a penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {:?}", six);
    println!("none is {:?}", none);

    //let sum = x + y;
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let coin = Coin::Dime;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("state quarter from {:?}", state);
    } else {
        count += 1;
    }
    println!("count is {}", count);

    let u8_value = 5u8;
    match u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let color = Color(20, 122, 200);
    if let Color(r, 125, b) = color {
        println!("r = {}, g = {}, b = {}", r, 125, b);
    }
    println!("r = {}", color.0);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}