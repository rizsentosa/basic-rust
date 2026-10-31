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

trait HumanSayGoodBye {
    fn good_bye(&self, name: &str) -> String;
}

/**
 * here is a trait HumanActivity
 * that extends Human and HumanSayGoodBye
 */
trait HumanActivity: Human + HumanSayGoodBye {}

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

impl HumanSayGoodBye for Person {
    fn good_bye(&self, name: &str) -> String {
        format!("{} says to , good bye, {}", self.first_name, name)
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

    // default trait
    fn version(&self) -> String {
        String::from("1.0.0")
    }
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

fn reference_human_call(human: &impl Human) {
    println!("{}", human.speak("rizqi"));
}

fn human_say_good_bye(human: &(impl Human + HumanSayGoodBye)) {
    println!("{}", human.speak("rizqi"));
    println!("{}", human.good_bye("rizqi"));
}

#[test]
fn test_trait_1() {
    let p1 = Person {
        id: String::from("1"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
        gender: 'M',
        height: 1.80,
        weight: 80.0,
    };
    let p2 = Person {
        id: String::from("2"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
        gender: 'M',
        height: 1.80,
        weight: 80.0,
    };
    let p3 = Person {
        id: String::from("3"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
        gender: 'M',
        height: 1.80,
        weight: 80.0,
    };
    println!("{}", p1.speak("lena"));
    println!("{}", p1.walk("tower garden"));
    println!("{}", p1.eat("apple"));
    p1.sleep();

    println!();

    reference_human_call(&p1);
    human_say_good_bye(&p1);

    println!();

    let mut list_person = ListedPerson { data: vec![] };
    let person1: Person = list_person.create(p1);
    list_person.create(p2);
    list_person.create(p3);

    println!("{}", list_person.version());
    println!("create: {:?}", person1);
    println!("read: {:#?}", list_person.read());
    println!("update: {:?}", list_person.update("1", person1));
    println!("delete: {:?}", list_person.delete("1"));
}

use chrono::{DateTime, Local, NaiveDate, NaiveTime};

#[derive(Debug, Clone)]
struct Post {
    id: String,
    title: String,
    description: String,
    content: String,
    timestamp: DateTime<Local>,
}

#[derive(Debug)]
struct APIResponse<T> {
    data: T,
    status_code: u16,
    message: String,
}

impl<T> APIResponse<T> {
    fn new(data: T, status_code: u16, message: String) -> Self {
        APIResponse {
            data,
            status_code,
            message,
        }
    }

    fn get_data(&self) -> &T {
        &self.data
    }

    fn get_message(&self) -> &String {
        &self.message
    }

    fn get_status_code(&self) -> &u16 {
        &self.status_code
    }
}

#[derive(Debug)]
enum Value<T> {
    NONE,
    DATA(T),
}

#[test]
fn post_get() {
    let date = chrono::Local::now();
    let post_1: Post = Post {
        id: "1".to_string(),
        title: "Ex adipisicing".to_string(),
        description: "Fugiat Lorem".to_string(),
        content: "Ex aliquip ea deserunt sint dolor".to_string(),
        timestamp: date,
    };
    let post_response: APIResponse<Post> = APIResponse {
        data: post_1,
        status_code: 200,
        message: "success".to_string(),
    };
    println!("{:#?}", post_response);
    println!("{:?}", post_response.get_data());
}

#[test]
fn generic_enum() {
    let value: Value<String> = Value::DATA(String::from("rizqi"));

    match value {
        Value::NONE => {
            println!("none");
        }
        Value::DATA(data) => {
            println!("value: {}", data);
        }
    }
}

#[derive(Debug)]
struct Identity<T: Human> {
    info: T, // generic bound
}

#[test]
fn identity_test() {
    let p1 = Person {
        id: String::from("1"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
        gender: 'M',
        height: 1.80,
        weight: 80.0,
    };
    let identity: Identity<Person> = Identity { info: p1 };
    println!("{:?}", identity);
}

// generic function
fn max<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 > v2 { v1 } else { v2 }
}

#[test]
fn test_max() {
    println!("{}", max(1, 2));
    println!("{}", max("a", "b"));
}
/**
 * T -> entity
 * K -> key
 */
trait SkeletonCrud<K, T = Self>
where
    T: Clone,
{
    fn create(&mut self, data: T) -> T;
    fn read(&self) -> Vec<T>;
    fn find(&self, id: K) -> Option<T>;
    fn update(&mut self, id: K, data: T) -> Option<T>;
    fn delete(&mut self, id: K) -> Option<T>;
}

struct DefineListForAny<T> {
    data: Vec<T>,
}

impl<T> DefineListForAny<T> {
    fn new() -> Self {
        DefineListForAny { data: vec![] }
    }
}

impl SkeletonCrud<String, Post> for DefineListForAny<Post> {
    fn create(&mut self, data: Post) -> Post {
        self.data.push(data.clone());
        data
    }
    fn read(&self) -> Vec<Post> {
        if self.data.is_empty() {
            return vec![];
        }
        self.data.clone()
    }
    fn find(&self, id: String) -> Option<Post> {
        let exists: Option<Post> = self.data.iter().find(|p| p.id == id).cloned();

        if exists.is_none() {
            return None;
        }

        let post: Post = exists.unwrap();
        Some(post)
    }
    fn update(&mut self, id: String, data: Post) -> Option<Post> {
        let slot: Option<&mut Post> = self.data.iter_mut().find(|p| p.id == id);
        if slot.is_none() {
            return None;
        }
        let slot: &mut Post = slot.unwrap();
        *slot = data.clone();
        Some(data)
    }
    fn delete(&mut self, id: String) -> Option<Post> {
        let index: usize = self.data.iter().position(|p| p.id == id).unwrap();
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }
}

#[test]
fn test_crud_post() {
    let mut list: DefineListForAny<Post> = DefineListForAny::new();

    let post1: Post = Post {
        id: String::from("1"),
        title: String::from("Ex adipisicing"),
        description: String::from("Fugiat Lorem"),
        content: String::from("Ex aliquip ea deserunt sint dolor"),
        timestamp: chrono::Local::now(),
    };

    let post2: Post = Post {
        id: String::from("2"),
        title: String::from("Ex adipisicing"),
        description: String::from("Fugiat Lorem"),
        content: String::from("Ex aliquip ea deserunt sint dolor"),
        timestamp: chrono::Local::now(),
    };

    let post2_updated: Post = Post {
        id: String::from("2"),
        title: String::from("perbarui postingan id 2"),
        description: String::from("Fugiat Lorem"),
        content: String::from("Ex aliquip ea deserunt sint dolor"),
        timestamp: chrono::Local::now(),
    };

    // masukkan data ke dalam vector list
    list.create(post1);
    list.create(post2);

    println!();
    println!("{:#?}\n", list.read());

    println!();
    println!("find_post: {:#?}\n", list.find("1".to_string()));

    println!();
    println!(
        "update_post: {:#?}\n",
        list.update("2".to_string(), post2_updated)
    );

    println!();
    println!("delete_post: {:#?}\n", list.delete("1".to_string()));
}

use core::ops::{Add, Mul, Rem};

struct Fruit {
    name: String,
    quantity: u32,
    price: f32,
    total_price: f64,
}
impl Mul for Fruit {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.price as f64 * rhs.quantity as f64
    }
}

#[test]
fn test_mul() {
    let apple: Fruit = Fruit {
        name: String::from("apple"),
        quantity: 2,
        price: 2.0,
        total_price: 0.0,
    };

    let banana: Fruit = Fruit {
        name: String::from("banana"),
        quantity: 4,
        price: 3.0,
        total_price: 0.0,
    };

    let total_price: f64 = apple * banana;
    println!("total price: {:.2}", total_price);
}

use std::cmp::{Ordering, PartialEq, PartialOrd};

impl PartialEq for Fruit {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_partial_cmp() {
    let apple: Fruit = Fruit {
        name: String::from("apple"),
        quantity: 10,
        price: 2.0,
        total_price: 0.0,
    };

    let banana: Fruit = Fruit {
        name: String::from("banana"),
        quantity: 4,
        price: 3.0,
        total_price: 0.0,
    };

    let quantity_is_equal = apple == banana;
    println!("quantity is equal: {}", quantity_is_equal);

    let quantity_is_greater = apple > banana;
    println!("quantity is greater: {}", quantity_is_greater);
}

// #[derive(Debug)]
struct Joke {
    id: u32,
    question: &'static str,
    answer: &'static str,
    sensitive_data: Option<&'static str>,
}

use std::collections::VecDeque;
use std::fmt::Debug;
impl Debug for Joke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Joke")
            .field("id", &self.id)
            .field("question", &self.question)
            .field("answer", &self.answer)
            .finish()
    }
}

#[test]
fn test_joke() {
    let joke: Joke = Joke {
        id: 1,
        question: "mouse apa yang gerak",
        answer: "mouse got",
        sensitive_data: Some("qwert==="),
    };
    println!("{:#?}", joke);
}

// code example how define clouser
fn str_convert_to_upper(v: &str, upper: fn(String) -> String) -> String {
    let result = upper(v.to_string());
    result
}

fn calculator<F>(a: i32, b: i32, operation: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    operation(a, b)
}

#[derive(Clone)]
struct Product {
    id: &'static str,
    name: &'static str,
    price: i32,
    quantity: i32,
}

struct ListedProduct {
    product_list: Vec<Product>,
}

impl Debug for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Product")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("price", &self.price)
            .field("qty", &self.quantity)
            .finish()
    }
}

impl SkeletonCrud<String, Product> for ListedProduct {
    fn create(&mut self, data: Product) -> Product {
        let data_send = data.clone();
        self.product_list.push(data_send);
        data
    }
    fn read(&self) -> Vec<Product> {
        if self.product_list.len() < 1 {
            return vec![];
        }
        self.product_list.clone()
    }
    fn find(&self, id: String) -> Option<Product> {
        let product: Option<Product> = self.product_list.iter().find(|item| item.id == id).cloned();
        if product.is_none() {
            return None;
        }
        Some(product.unwrap())
    }
    fn update(&mut self, id: String, data: Product) -> Option<Product> {
        let wrap_product: Option<&mut Product> =
            self.product_list.iter_mut().find(|item| item.id == id);
        if wrap_product.is_none() {
            return None;
        }
        let mut slot: &mut Product = wrap_product.unwrap();
        *slot = data;
        Some(slot.clone())
    }
    fn delete(&mut self, id: String) -> Option<Product> {
        let index: usize = self
            .product_list
            .iter()
            .position(|item| item.id == id)
            .unwrap();
        Some(self.product_list.remove(index))
    }
}

#[test]
fn test_closure() {
    let sum: fn(i32, i32) -> i32 = |x, y| -> i32 { x + y };
    let result = sum(2, 4);
    assert_eq!(result, 6);

    let os: &str = "linux kernel";
    let os_upper: String =
        str_convert_to_upper(os, |value: String| -> String { value.to_uppercase() });
    assert_eq!(os_upper, "LINUX KERNEL");

    println!("{}", os_upper);

    // calculator clouser
    let add = |a: i32, b: i32| a + b;
    let multiply = |a: i32, b: i32| a * b;
    let substance = |a: i32, b: i32| a - b;

    assert_eq!(calculator(2, 4, add), 6);
    assert_eq!(calculator(2, 4, multiply), 8);
    assert_eq!(calculator(2, 4, substance), -2);

    let price_usd: Vec<u32> = vec![10, 20, 50];
    let curs_idr = 15000;
    let price_idr: Vec<u32> = price_usd.iter().map(|v| v * curs_idr).collect();
    assert_eq!(price_idr, vec![150000, 300000, 750000]);

    let mut list_product = vec![
        Product {
            id: "lp-1",
            name: "laptop",
            price: 1000000,
            quantity: 10,
        },
        Product {
            id: "lp-2",
            name: "monitor",
            price: 500000,
            quantity: 5,
        },
        Product {
            id: "lp-3",
            name: "keyboard",
            price: 100000,
            quantity: 20,
        },
    ];

    list_product.retain(|product| product.quantity > 5);
    println!("{:#?}", list_product);
}

// ada 3 category tipe data collection
// 1. sequence, 2. maps, 3. set
/*
- Sequence
- Vec<T> Koleksi paling umum. Menyimpan daftar nilai dinamis dengan tipe data sama di blok memori bersambung
- 2. VecDeque<T> Antrean dua arah. Sangat efisien untuk menambah atau menghapus elemen dari awal maupun akhir daftar
- 3. LinkedList<T> Setiap elemen menyimpan pointer ke elemen sebelum dan sesudahnya (jarang digunakan di Rust karena Vec biasanya lebih efisien di CPU modern)
*/

// Vec<T> studi kasus keranhang belanja sederhana
#[test]
fn test_cart_shopping() {
    let mut cart: Vec<Product> = Vec::new();
    // adding element to cart
    cart.push(Product {
        id: "p-1",
        name: "laptop",
        price: 1000000,
        quantity: 10,
    });
    cart.push(Product {
        id: "p-2",
        name: "monitor",
        price: 500000,
        quantity: 5,
    });

    // print cart
    println!("1.{:#?}", cart);

    // update cart quantity
    for product in &mut cart {
        if product.id == "p-1" {
            product.quantity = product.quantity + 10
        }
    }

    // print cart
    println!("2.{:#?}", cart);

    // hitung total price yang ada di dalam cart
    let total: i32 = cart.iter().map(|item| item.price).sum();
    println!("total price: {}", total);

    // delete product yang terakhir dimasukkan
    cart.pop();

    // print cart
    println!("3.{:#?}", cart);
}

// VecDeque<T> studi kasus antrean proses transaksi bank sederhana
struct Transaction {
    id: u32,
    send: &'static str,
    amount: u64,
}

impl Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction")
            .field("id", &self.id)
            .field("send", &self.send)
            .field("amount", &self.amount)
            .finish()
    }
}

#[test]
fn test_queue_bank() {
    // membuat antrean kosong
    let mut queue_trancation: VecDeque<Transaction> = VecDeque::new();

    // adding queue to transaction
    queue_trancation.push_back(Transaction {
        id: 101,
        send: "budi",
        amount: 500_000,
    });
    queue_trancation.push_back(Transaction {
        id: 102,
        send: "siti",
        amount: 400_000,
    });
    queue_trancation.push_front(Transaction {
        id: 103,
        send: "andi",
        amount: 300_000,
    });

    let mut other_queue_bank: VecDeque<Transaction> = VecDeque::new();
    other_queue_bank.push_back(Transaction {
        id: 104,
        send: "joko",
        amount: 200_000,
    });

    // menggabungkan antrean
    queue_trancation.append(&mut other_queue_bank);

    println!("jumlah antrian saat ini {}", queue_trancation.len());
    println!();

    println!("1.{:#?}", queue_trancation);

    println!();

    println!("memulai pemrosesan transaksi...");
    while let Some(data) = queue_trancation.pop_front() {
        println!(
            "memproses id {} transfer rp. {} dari {}",
            data.id, data.amount, data.send
        );
    }
    println!("jumlah antrian saat ini {}", queue_trancation.len());
}

fn print_username_borrow(param1: &mut String) -> String {
    param1.push_str(" updated");
    param1.to_string()
}

fn print_username_not_borrow(param1: String) -> String {
    param1
}

fn print_stack(param1: u32) -> () {
    println!("param1: {}", param1);
}

#[test]
fn owner_ship_sample() {
    let num1: u32 = 10; // value data num1 disimpan didalam stack
    let name: String = String::from("arado"); // value data name disimpan didalam heap

    print_stack(num1);
    println!("num1: {}", num1);

    // ownership name dipindah ke print_username_borrow
    // kemudian hasil print_username_borrow disimpan didalam variable res1 yang dimana ini pemilik ownership yang baru
    let res1 = print_username_not_borrow(name);

    let mut fname = String::from("arado");

    let mut res2 = print_username_borrow(&mut fname);

    println!("old {}, new {}", res1, res2);

    let mut product: Vec<Product> = Vec::new();

    product.push(Product {
        id: "p-1",
        name: "laptop",
        price: 1000000,
        quantity: 10,
    });

    product
        .iter()
        .for_each(|item: &Product| println!("{:?}", item));
}

#[test]
fn test_product_list() {
    // buat list kosong
    let mut list_product: ListedProduct = ListedProduct {
        product_list: Vec::new(),
    };

    // start 2 data ini akan dimasukkan kedalam list_product
    let p1 = Product {
        id: "p-1",
        name: "laptop",
        price: 1000000,
        quantity: 10,
    };

    let p2 = Product {
        id: "p-2",
        name: "monitor",
        price: 300000,
        quantity: 20,
    };

    // memasukkan data ke dalam list_product ownership
    list_product.create(p1);
    list_product.create(p2);

    println!();
    println!(
        "find data p-1: {:#?}",
        list_product.find("p-1".to_string()).unwrap()
    );

    // iterasi data
    println!();
    println!("product list: {:#?}", list_product.read());

    // update data
    let p2_update = Product {
        id: "p-2",
        name: "monitor samsung",
        price: 500000,
        quantity: 8,
    };
    list_product.update("p-2".to_string(), p2_update);

    // iterasi data
    println!();
    println!("after update product list: {:#?}", list_product.read());

    // delete data
    list_product.delete("p-1".to_string());
    println!();
    println!(
        "data yang tersisa count={}",
        list_product.product_list.len()
    );
}
