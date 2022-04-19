use lazy_static::lazy_static;

lazy_static! {
    pub static ref BASE_URL: String = std::env::var("BASE_URL")
        .expect("`BASE_URL` must be set");
}

lazy_static! {
    pub static ref PORT: usize = std::env::var("PORT")
        .expect("`PORT` must be set")
        .parse()
        .expect("`PORT` must be a positive integer");
}

lazy_static! {
    pub static ref DATABASE_URL: String = std::env::var("DATABASE_URL")
        .expect("`DATABASE_URL` must be set");
}
