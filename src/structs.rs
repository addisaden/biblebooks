pub struct BookCollection {
    pub title: &'static str,
    pub books: &'static[Book],
    pub color: &'static str,
    pub lang: &'static str,
}

pub struct Book {
    pub title: &'static str,
}

