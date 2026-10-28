pub fn run() {
    let x = 10;
    let x = x + 1; // 11
    // here x is shadowed
    {
        let x = x * 2; // 22
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
