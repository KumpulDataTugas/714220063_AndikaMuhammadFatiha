struct Book {
    title: String,
    pages: u32,
}

fn print_book(book: &Book) {
    println!("Title: {}, Pages: {}", book.title, book.pages);
}

fn main() {
    let book = Book {
        title: String::from("Rust Essentials"),
        pages: 320,
    };
    print_book(&book);
}