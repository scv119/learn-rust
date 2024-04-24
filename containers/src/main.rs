use std::collections::HashMap;

fn vectors () {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
    let v = vec![3, 2, 1];
    println!("{:?}", v);

    let third = v[2];
    println!("{}", third);

    let third: Option<&i32> = v.get(2);
    if let Option::Some(val) = third {
        println!("{}", val);
    }
    match third {
        Option::Some(val) => println!("{}", val),
        Option::None => println!("None"),
    }
    let does_not_exist = v.get(100);
    if let Option::Some(val) = does_not_exist {
        println!("{}", val);
    }

    let mut v = vec![100, 32, 57];
    
    for mut i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_vecs() {
    let v = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Float(2.0), SpreadsheetCell::Text(String::from("blue"))];
    dbg!(v);
}

fn test_strings() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::new();
    s.push_str("hello");
    dbg!(&s);
    s.push('l');
    dbg!(&s);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = String::from("what");

    // let s3 = s1 + &s2;
    let s = format!("{s1}-{s2}-{s3}");
    dbg!(s);

    let hello = "Здравствуйте";
    let s = &hello[0..2];
    dbg!(s);

    for c in "Зд".bytes() {
        println!("{c}");
    }
}

fn maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    // let team_name = String::from("Blue");
    // let score = scores.get(&String::from("Blue")).unwrap_or(&0);
    let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("score of Blue is {}",score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 25);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    dbg!(&scores);
}

fn count_letters() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    println!("Hello, world!");
   // vectors();
   // enum_vecs();
    // test_strings();
    // maps();
    count_letters();
}
