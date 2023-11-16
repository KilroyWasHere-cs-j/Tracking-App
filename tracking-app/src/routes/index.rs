#[get("/")]
pub fn index() -> String {
    format!("The config value is: {}", "")
}
