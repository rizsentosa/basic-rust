use std::io;
use std::mem;

#[derive(Debug, Clone)]
struct Book {
    title: String,
    description: String,
    price: f64,
    stock: u16,
    sale: bool,
    tags: Vec<String>,
}

struct BookStore {
    books: Vec<Book>,
}

impl BookStore {
    // constructor
    fn new() -> Self {
        BookStore { books: Vec::new() }
    }

    // menghitung estimasi penggunaan memory
    fn memory_usage(&self) -> usize {
        let mut total = mem::size_of::<BookStore>();
        total += self.books.capacity() * mem::size_of::<Book>();
        for book in &self.books {
            total += book.title.capacity();
            total += book.description.capacity();
            total += book.tags.capacity() * mem::size_of::<String>();

            for tag in &book.tags {
                total += tag.capacity();
            }
        }

        total
    }

    // menampikan informasi memory
    fn print_memory_info(&self) {
        println!("\nInformasi Memory:");
        println!("{}", "=".repeat(50));
        println!("Jumlah books          : {}", self.books.len());
        println!("Capacity Vec<Books>   : {}", self.books.capacity());
        println!(
            "SizeOf BookStore      : {} bytes",
            mem::size_of::<BookStore>()
        );
        println!("SizeOf Books          : {} bytes", mem::size_of::<Book>());
        println!(
            "Estimasi total memory : {} bytes (~{:.2} KB)",
            self.memory_usage(),
            self.memory_usage() as f64 / 1024.0
        );
        println!("{}", "=".repeat(50));
    }

    // memasukkan data buku kedalam collection books
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
        println!("buku berhasil ditambahkan");
    }

    // menampilkan data semua buku dari collection books
    fn read_book(&self) {
        if self.books.is_empty() {
            println!("tidak ada buku dalam arsip");
            return;
        }
        println!("daftar buku");
        println!("{}", "=".repeat(80));
        for (index, book) in self.books.iter().enumerate() {
            println!("Book #{}", index + 1);
            println!("Title: {}", book.title);
            println!("Description: {}", book.description);
            println!("Price: Rp {:.2}", book.price);
            println!("Stock: {}", book.stock);
            println!("Sale: {}", if book.sale { "Ya" } else { "Tidak" });
            println!("tags: {}", book.tags.join(", "));
            println!("{}", "-".repeat(80));
        }
    }

    // mencari data buku berdasarkan judul
    fn find_by_title_book(&self, title: &str) -> Option<&Book> {
        self.books
            .iter()
            .find(|book| book.title.to_lowercase().contains(&title.to_lowercase()))
    }

    // memperbarui data buku berdasarkan index
    fn update_book(&mut self, index: usize, updated_book: Book) -> Result<(), String> {
        if index < self.books.len() {
            self.books[index] = updated_book;
            Ok(())
        } else {
            Err(format!(
                "index {} tidak valid total book: {}",
                index,
                self.books.len(),
            ))
        }
    }

    // hapus data buku berdasarkan index
    fn delete_book(&mut self, index: usize) -> Result<Book, String> {
        if index < self.books.len() {
            Ok(self.books.remove(index))
        } else {
            Err(format!(
                "index {} tidak valid total book: {}",
                index,
                self.books.len(),
            ))
        }
    }

    // total keseluruhan buku
    fn count(&self) -> usize {
        self.books.len()
    }
}

fn read_input_user(prompt: &str) -> String {
    print!("{}", prompt);

    io::Write::flush(&mut io::stdout()).expect("gagal melakukan flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("gagal membaca input");
    input.trim().to_string()
}

pub fn run_app_book() {
    let mut store = BookStore::new();

    let sample_book1 = Book {
        title: "Belajar Rust".to_string(),
        description: "Panduan lengkap belajar pemrograman Rust".to_string(),
        price: 150000.0,
        stock: 10,
        sale: true,
        tags: vec!["programming".to_string(), "rust".to_string()],
    };
    let sample_book2 = Book {
        title: "Belajar Bahasa Jepang".to_string(),
        description: "In occaecat ex qui amet Lorem sit sint.".to_string(),
        price: 180000.0,
        stock: 12,
        sale: true,
        tags: vec!["edukasi".to_string(), "jepang".to_string()],
    };

    store.add_book(sample_book1);
    store.add_book(sample_book2);

    println!("selamat datang di book store management system!...");
    loop {
        println!("\n📋 Menu:");
        println!("1. Lihat semua books");
        println!("2. Cari book");
        println!("3. Tambah book baru");
        println!("4. Update book");
        println!("5. Hapus book");
        println!("6. Informasi Memory Usage");
        println!("7. Keluar");

        let choice = read_input_user("\nPilih Menu (1-6): ");
        match choice.as_str() {
            "1" => {
                store.read_book();
            }
            "2" => {
                let title = read_input_user("masukkan judul buku yang dicari: ");
                if title.is_empty() {
                    println!("judul buku tidak boleh kosong");
                    continue;
                }
                match store.find_by_title_book(&title) {
                    Some(book) => {
                        println!("buku ditemukan");
                        println!("{:#?}", book);
                    }
                    None => {
                        println!("buku tidak ditemukan");
                    }
                }
            }
            "3" => {
                println!("tambah buku baru");
                let title: String = read_input_user("Title: ");
                let description: String = read_input_user("Description: ");

                let price_input: String = read_input_user("Price: ");
                let price: f64 = price_input.parse().unwrap_or_else(|_| {
                    println!("input tidak valid, menggunakan default 0");
                    0.0
                });

                let stock_input: String = read_input_user("Stock: ");
                let stock: u16 = stock_input.parse().unwrap_or_else(|_| {
                    println!("input tidak valid, menggunakan default 0");
                    0
                });

                let sale_input: String = read_input_user("Sale? (ya/tidak): ");
                let sale: bool = sale_input.to_lowercase() == "ya";

                let tags_input = read_input_user("tags (pisahkan dengan koma): ");
                let tags: Vec<String> = if tags_input.is_empty() {
                    Vec::new()
                } else {
                    tags_input
                        .split(",")
                        .map(|tag| tag.trim().to_string())
                        .collect()
                };

                let new_book = Book {
                    title,
                    description,
                    price,
                    stock,
                    sale,
                    tags,
                };
                store.add_book(new_book);
            }
            "4" => {
                store.read_book();
                let index_input = read_input_user("\nmasukkan nomor buku yang ingin di perbarui: ");
                match index_input.parse::<usize>() {
                    Ok(index) => {
                        if index > 0 && index <= store.count() {
                            println!("\n perbarui buku #{}", index);
                            let title = read_input_user("Title: ");
                            let description = read_input_user("Description: ");

                            let price_input = read_input_user("Price: ");
                            let price: f64 = price_input.parse().unwrap_or_else(|_| 0.0);

                            let stock_input = read_input_user("Stock: ");
                            let stock: u16 = stock_input.parse().unwrap_or_else(|_| 0);

                            let sale_input = read_input_user("Sale? (ya/tidak): ");
                            let sale: bool = sale_input.to_lowercase() == "ya";

                            let tags_input = read_input_user("Tags (pisahkan dengan koma): ");
                            let tags: Vec<String> = if tags_input.is_empty() {
                                Vec::new()
                            } else {
                                tags_input
                                    .split(",")
                                    .map(|tag| tag.trim().to_string())
                                    .collect()
                            };

                            let updated_book = Book {
                                title,
                                description,
                                price,
                                stock,
                                sale,
                                tags,
                            };
                            match store.update_book(index - 1, updated_book) {
                                Ok(_) => println!("buku berhasil diperbarui!"),
                                Err(e) => println!("{}", e),
                            }
                        } else {
                            println!("nomor buku tidak valid");
                        }
                    }
                    Err(_) => {
                        println!("input harus berupa angka!");
                    }
                }
            }
            "5" => {
                store.read_book();
                let index_input = read_input_user("masukkan nomor buku yang ingin dihapus: ");
                match index_input.parse::<usize>() {
                    Ok(index) => {
                        if index > 0 && index <= store.count() {
                            match store.delete_book(index - 1) {
                                Ok(book) => {
                                    println!("buku \"{}\" berhasil dihapus!", book.title)
                                }
                                Err(e) => {
                                    println!("{}", e);
                                }
                            }
                        } else {
                            println!("nomor buku tidak valid!");
                        }
                    }
                    Err(_) => {
                        println!("input harus berupa angka!");
                    }
                }
            }
            "6" => {
                store.print_memory_info();
            }
            "7" => {
                println!("👋 Terima kasih telah menggunakan Book Store!");
                break;
            }
            _ => {
                println!("pilihan tidak valid");
            }
        }
    }
}
