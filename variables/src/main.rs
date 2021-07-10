fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let f1: f32 = 0.23;
    let f2: f64 = 0.24;
    let f3 = 0.25;

    println!("The value of f1 is {}", f1);
    println!("The value of f2 is {}", f2);
    println!("The value of f3 is {}", f3);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces is {}", spaces);

    println!("43 % 5 = {}", 43 % 5);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //let (x, y, z) = tup;

    println!("tup as string is {}, {}, {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 4, 5, 6];

    println!("arr as as string is {}", arr[0]);

    let arr2: [bool; 4] = [true, false, false, true];
    println!("arr2 as as string is {}", arr2[0]);

    let arr3 = [123; 4];
    println!("arr3 as as string is {}", arr3.len());
}
