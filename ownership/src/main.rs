/*fn takes_and_gives_back(a_string: String) -> String {
    a_string
}*/


/*fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}*/
fn main() {
    /* What is ownership
    let mut str = String::from("hello");

    str.push_str(", world!");

    println!("{}", str);

    {
        let s = String::from("hello");
        println!("{}", s);
        //calls drop when scope ends
    }


    //let x = 5;
    //let y = x;

    let s1 = String::from ("hello");
    let s2 = s1;
    println!("{}, world!", s2);// s1 won't work

    let s3 = String::from ("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);// s1 won't work

    let test_str = String::from("hi");
    let test_str2 = takes_and_gives_back(test_str);
    println!("{}", test_str2);
    
    */

    /* References and borrowing 
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    {   
        let r1 = &mut s;
    }
    let r2 = &mut s;

    println!("{}, {}", r2, r2);

    change(&mut s);
    println!("{}", s);

    let ref_to_nothing = dangle();

    */

    /*
        The slice type
    */
    let s = String::from("hello world");

    let word = first_word(&s);

    //let my_string_literal = "hello world";
    //let word = first_word(my_string_literal);

    println!("The first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}