#[allow(dead_code)]
#[derive(Debug)]
pub struct FavoriteColor {
    pub color: String,
    pub color_code: String,
    pub color_hex: String,
}

impl FavoriteColor {
    fn see_color(&self) -> String {
        format!(
            "my favorite color is {}, color code is {}, color hex is {}",
            self.color, self.color_code, self.color_hex
        )
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn initial(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn double_height(&self) -> u32 {
        self.height * 2
    }
}

#[test]
fn run_method() {
    let rect1 = Rectangle {
        width: 10,
        height: 15,
    };

    let rect2 = Rectangle {
        width: 3,
        height: 5,
    };

    let rect3 = Rectangle::initial(60, 45);

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!();
    println!();

    println!("{:?}", rect1);
    println!("{:?}", rect2);
    println!("{:?}", rect3);
    println!(
        "area: {}, double height {}",
        rect1.area(),
        rect1.double_height()
    );
    println!("the rectangle has a nonzero width {}", rect1.width());
}

#[allow(dead_code)]
fn run_color() {
    let color = FavoriteColor {
        color: String::from("blue"),
        color_code: String::from("#0000ff"),
        color_hex: String::from("0000ff"),
    };
    println!("1.{:?}", color);
    println!("2.{}", color.see_color());
}

#[allow(dead_code)]
fn refer_and_defer() {
    let title = String::from("i like pusy");
    let ref_title = &title;
    println!("title: {}", title);
    println!("ref_title: {}", ref_title);

    let mut x = 5;
    println!("{}", x); // Output: 5

    let mut y = &mut x;
    println!("{}", y); // Error! y adalah alamat, bukan nilai.
    println!("{}", *y); // Output: 5 (Kita "buka" alamat y untuk dapat nilainya)

    *y = 10;
    println!("{}", x); // Output: 10
}
