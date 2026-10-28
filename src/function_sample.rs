#[allow(dead_code)]
pub(crate) fn another_function(x: u32) {
    let y: u32 = {
        let z: u32 = 3;
        x + z
    };

    println!("another function {y}");
}

#[allow(dead_code)]
pub(crate) fn print_label_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}

pub fn greeting() -> String {
    return "hello greetings".to_string();
}

pub fn plust_one(value: i32) -> i32 {
    value + 1
}

pub fn happy() {
    println!("happy");
}

pub fn sad() {
    println!("sad");
}

pub fn divisible(value: u32) {
    if value % 4 == 0 {
        println!("number is divisible by 4");
    } else if value % 3 == 0 {
        println!("number is divisible by 3");
    } else if value % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

#[test]
fn call_me() {
    println!("you call me !!");
}
