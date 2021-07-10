fn main() {
    let number = 8;

    if number < 5 {
        println!("Expression was true");
    } else {
        println!("Expression was false");
    }

    let condition = false;
    let num = if condition { 5 } else { 6 };

    println!("num = {}", num);
}
