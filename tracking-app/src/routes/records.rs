#[get("/records/<username>/<passwordhash>")]
pub fn getRecords(username: String, passwordhash: String) -> String{
    format!("Hello, {}! Your password hash is {}", username, passwordhash)
}

#[get("/records/<username>/<passwordhash>/recordtype/<recordtype>")]
pub fn getRecordsByType(username: String, passwordhash: String, recordtype: String) -> String{
    format!("Hello, {}! Your password hash is {}. You want records of type {}", username, passwordhash, recordtype)
}
