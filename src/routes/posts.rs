#[get("/post")]
pub fn get_post() -> &'static str {
    "Post!"
}