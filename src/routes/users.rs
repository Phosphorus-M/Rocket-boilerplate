    #[get("/user")]
    pub fn hello() -> &'static str {
        "Hello user!"
    }