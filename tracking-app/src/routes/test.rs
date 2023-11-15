#[get("/test")]
pub fn test() -> String {
    format!("{}", "Hello World")
}
