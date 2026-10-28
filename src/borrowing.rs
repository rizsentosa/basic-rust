pub fn init() {
    println!("borrowing...");
}

#[test]
fn run() {
    println!("[1] borrowing modules");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of {s1} is {len}");

    println!("======================");

    println!();

    println!("[2] mutable borrowing modules");

    let mut s_value: String = String::from("hello");
    let s_result: String = change(&mut s_value);
    println!("{s_result}");
    println!("======================");

    println!();

    let mut coffe_shop: String = String::from("agera coffe shop");
    let cp_coffe1: &String = &coffe_shop;
    let cp_coffe2: &String = &coffe_shop;
    println!("{cp_coffe1}, {cp_coffe2}");
    let cp_coffe3: &mut String = &mut coffe_shop;
    cp_coffe3.push_str(" updated");
    println!("{cp_coffe3}");

    println!();
    println!("======================");

    let word: String = String::from("cambagga red");
    let word1: &str = first_word(&word[0..6]);
    let word2: &str = first_word(&word[..]);
    let word_red: &str = &word[9..word.len()];
    println!("{word1}, {word_red}, {word2}");
}

#[allow(dead_code)]
fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

#[allow(dead_code)]
fn dangle() -> String {
    let value = String::from("let's drink coffe");
    value
}

#[allow(dead_code)]
fn change(some_string: &mut String) -> String {
    some_string.push_str(", world");
    return some_string.to_string();
}

#[allow(dead_code)]
fn calculate_length(s: &String) -> usize {
    return s.len();
}
