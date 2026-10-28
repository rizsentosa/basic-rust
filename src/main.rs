#![allow(unused_imports)]
// mod guess;
// mod shadowing;
// mod borrowing;
// mod function_sample;
mod book_store;
mod first;
mod methods_sample;
mod model;
mod third;

fn main() {}

#[allow(dead_code)]
fn run_app_book() {
    book_store::run_app_book();
}

use first::first_library as f1;
use first::seconded::thirded::call_library as f2;
#[test]
fn test_first_library() {
    f1();
    println!();
    f2();
}

#[test]
fn scenario_success() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn datatype_test() {
    // Scalar Types
    // ini bisa negative dan positive
    let _x1: i8 = -127;
    let _x2: i16 = -32767;
    let _x3: i32 = 2147483647;
    let _x4: i64 = 9223372036854775807;

    // hanya positive
    let _y1: u8 = 255;
    let _y2: u16 = 65535;
    let _y3: u32 = 4294967295;
    let _y4: u64 = 18446744073709551615;

    // float value
    let _f1: f32 = 10.5;
    let _f2: f64 = 10.5;

    // boolean value
    let _b1: bool = true;
    let _b2: bool = false;

    // char value
    let _c1: char = 'z';

    // Compound Types
    // tupple
    let tup1: (u32, f32, bool, i8, &str) = (1000, 10.5, true, -10, "tupple hello");
    let (q, w, e, r, t) = tup1;
    println!("{q}, {w}, {e}, {r}, {t}");
    println!("{}, {}, {}, {}, {}", tup1.0, tup1.1, tup1.2, tup1.3, tup1.4);

    // array
    let months: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    for month in months {
        print!("{}, ", month);
    }
}

#[test]
fn return_value_loop() {
    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is: {result}");
    assert_eq!(result, 20);
}

#[test]
fn loop_label() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
}

#[test]
fn loop_with_while() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!...");
        number -= 1;
    }
    println!("[1].bye....");

    let fruits: [&str; 5] = ["banana", "apple", "grape", "watermelon", "orange"];
    let mut index: usize = 0;
    let search: &str = "watermelon";

    while index < fruits.len() {
        let message: String = if fruits[index] == search {
            format!("i like {}", fruits[index])
        } else {
            format!("i dont like {}", fruits[index])
        };
        println!("{message}");
        index += 1;
    }
    println!("[2].bye....");

    for number in (1..3).rev() {
        println!("rev: {number}");
    }
}

#[test]
fn twelve_days_of_christmas() {
    let gifts = [
        "seekor burung partridge di pohon pear",
        "dua ekor merpati",
        "tiga ekor ayam betina Prancis",
        "empat ekor burung memanggil",
        "lima cincin emas",
        "enam angsa bertelur",
        "tujuh angsa berenang",
        "delapan pelayan memerah susu",
        "sembilan penari menari",
        "sepuluh pangeran melompat",
        "sebelas pemain pipa bermain",
        "dua belas drummer drumming",
    ];

    let days = [
        "pertama",
        "kedua",
        "ketiga",
        "keempat",
        "kelima",
        "keenam",
        "ketujuh",
        "kedelapan",
        "kesembilan",
        "kesepuluh",
        "kesebelas",
        "keduabelas",
    ];

    for day in 0..12 {
        println!("pada hari {} natal, kekasihku memberiku: ", days[day]);

        for gift_index in (0..=day).rev() {
            if gift_index == day && day > 0 {
                println!("{} dan", gifts[gift_index]);
            } else if gift_index == 0 && day > 0 {
                println!("{}", gifts[gift_index]);
            } else {
                println!("{}", gifts[gift_index]);
            }
        }
        println!();
    }
}
#[test]
fn ownership_sample() {
    let s = &String::from("hello");

    take_ownership(s);

    let x: i32 = 10;
    makes_copy(x);

    println!("s: {s}");
}

#[allow(dead_code)]
fn take_ownership(some_string: &String) {
    println!("{some_string}");
}

#[allow(dead_code)]
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[test]
fn test_rectangle() {
    let scale: u32 = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 20,
    };
    dbg!(&rect1);
    println!("data_{:?}", rect1);
    println!("data_{:#?}", rect1);
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        println!("kamu melakukan pembayaran {}", amount);
        match self {
            Payment::CreditCard(number) => {
                println!("pembayaran dengan kartu kredit {}", number);
            }
            Payment::BankTransfer(bank, _) => {
                println!("pembayaran dengan transfer bank {}", bank);
            }
            Payment::EWallet(wallet, account) => {
                println!("pembayaran dengan e-wallet {} {}", wallet, account);
            }
        }
    }
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> Self {
        GeoPoint(long, lat)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum RoleType {
    ADMIN(String),
    USER(String),
}

type TUsername = String;
type TEmail = String;

#[allow(dead_code)]
struct Account {
    username: TUsername,
    email: TEmail,
    role: RoleType,
}

impl Account {
    fn new(username: String, email: String, role: RoleType) -> Self {
        Account {
            username,
            email,
            role,
        }
    }

    #[allow(dead_code)]
    fn do_something() {}
}

#[test]
fn test_payment_enum() {
    let _payment1: Payment = Payment::CreditCard(String::from("123443333"));
    let _payment2: Payment =
        Payment::BankTransfer(String::from("BCA"), String::from("888-333-222"));
    let _payment3: Payment = Payment::EWallet(String::from("DANA"), String::from("888-333-222"));

    _payment3.pay(4000000);
}

#[test]
fn pattern_matching() {
    let name: &str = "john";
    match name {
        "john" | "elric" => println!("john and elric"),
        "jane" => println!("Hello, Jane!"),
        _ => println!("Hello, someone else!"),
    }

    let value: u32 = 80;
    match value {
        0..=20 => println!("0..=20"),
        21..=60 => println!("21..=60"),
        61..=80 => println!("61..=80"),
        81..=100 => println!("81..=100"),
        _ => println!("other"),
    }

    println!();
    println!();

    let point: GeoPoint = GeoPoint::new(1.0, 0.0);
    match point {
        GeoPoint(long, 0.0) => println!("long: {long:.2}"),
        GeoPoint(0.0, lat) => println!("lat: {lat:.2}"),
        GeoPoint(long, lat) => println!("long: {long:.2}, lat: {lat:.2}"),
    }

    let admin: Account = Account::new(
        String::from("admin"),
        String::from("test@gmail.com"),
        RoleType::ADMIN(String::from("admin can do anything")),
    );
    match admin {
        Account { email, role, .. } => {
            println!("email: {email}, role: {role:?}");
        }
    }

    let guest: Account = Account::new(
        String::from("guest"),
        String::from("test@gmail.com"),
        RoleType::USER(String::from("only read data")),
    );
    match guest {
        Account { username, role, .. } => {
            println!("username: {username}, role: {role:?}");
        }
    }

    let option: &str = "4";
    match option.parse().unwrap() {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("other"),
    }

    let value: u32 = 3;
    let result: &str = match value {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("result: {result}");
}

#[allow(unused_imports)]
use methods_sample::FavoriteColor;
#[allow(unused_imports)]
use model::movies::TMovie as movie;
#[test]
fn access_module() {
    let color_favorite = FavoriteColor {
        color: String::from("blue"),
        color_code: String::from("#0000ff"),
        color_hex: String::from("0000ff"),
    };
    let movie: movie = movie {
        title: "Avengers".to_string(),
        description: "Fugiat culpa occaecat ea laborum mollit commodo in commodo amet in."
            .to_string(),
        year: 2020,
        rating: 8.0,
    };

    println!("{:#?}\n{:#?}", color_favorite, movie);
}

#[derive(Debug, Clone)]
struct Person {
    id: String,
    first_name: String,
    last_name: String,
    age: u8,
    gender: char,
    height: f32,
    weight: f32,
}

trait Human {
    fn speak(&self, name: &str) -> String;
    fn walk(&self, location: &str) -> String;
    fn eat(&self, food: &str) -> String;
    fn sleep(&self) -> ();
}

impl Human for Person {
    fn speak(&self, name: &str) -> String {
        format!("hello, {} I'm {}", name, self.first_name)
    }
    fn walk(&self, location: &str) -> String {
        format!("I'm walking to {}", location)
    }
    fn eat(&self, food: &str) -> String {
        format!("I'm eating {}", food)
    }
    fn sleep(&self) -> () {
        println!("I'm sleeping");
    }
}

struct ListedPerson {
    data: Vec<Person>,
}

trait Crud<T> {
    fn read(&self) -> Vec<T>;
    fn create(&mut self, data: T) -> T;
    fn update(&mut self, id: &str, data: T) -> Option<T>;
    fn delete(&mut self, id: &str) -> Option<T>;
}

impl Crud<Person> for ListedPerson {
    fn read(&self) -> Vec<Person> {
        self.data.clone()
    }

    fn create(&mut self, data: Person) -> Person {
        self.data.push(data.clone());
        data
    }

    fn update(&mut self, id: &str, data: Person) -> Option<Person> {
        let slot = self.data.iter_mut().find(|p| p.id == id)?;
        let old = std::mem::replace(slot, data);
        Some(old)
    }

    fn delete(&mut self, id: &str) -> Option<Person> {
        let idx = self.data.iter().position(|p| p.id == id)?;
        Some(self.data.remove(idx))
    }
}

#[test]
fn test_trait_1() {
    let person = Person {
        id: String::from("1"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
        gender: 'M',
        height: 1.80,
        weight: 80.0,
    };
    println!("{}", person.speak("lena"));
    println!("{}", person.walk("tower garden"));
    println!("{}", person.eat("apple"));
    person.sleep();

    println!();

    let mut list_person = ListedPerson { data: vec![] };
    let person1 = list_person.create(person);
    println!("create: {:?}", person1);
    println!("read: {:?}", list_person.read());
    println!("update: {:?}", list_person.update("1", person1));
    println!("delete: {:?}", list_person.delete("1"));
}
