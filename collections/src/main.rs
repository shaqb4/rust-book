use std::{collections::HashMap, hash::Hash};

fn main() {
    vecs();

    strings();

    hashmaps();
}

fn vecs() {
    let v: Vec<i32> = Vec::new();

    {
        let mut v2 = vec![1, 2, 3];
    
        v2.push(5);
        v2.push(6);
        v2.push(7);

        let third = &v2[2];
        println!("the third element is {}", third);

        match v2.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }

        for i in &v2 {
            println!("{}", i);
        }

        for i in &mut v2 {
            *i += 50;
        }

        for i in &v2 {
            println!("{}", i);
        }
    }
}

fn strings() {
    let data = "initial contents";

    //let s = String::new();
    //let s = data.to_string();
    let s = "initial contents".to_string();

    let mut s2 = String::from("foo");
    s2.push_str("bar");
    s2.push('!');

    let hello = String::from("Hello, ");
    let world = String::from("World");
    let hello_world = hello + &world; //hello was moved and can no longer be used.

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", tic_tac_toe);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

fn hashmaps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let blue_score = scores2.get(&team_name);
    if let Some(val) = blue_score {
        println!("{}", val);
    }

    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }

    scores2.entry(String::from("Yellow")).or_insert(20);


    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);
}