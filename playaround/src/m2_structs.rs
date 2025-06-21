#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let user1 = User {
            username: String::from("user1"),
            email: String::from("abc@hotmail.com"),
            sign_in_count: 1,
            active: true
        };
        dbg!(user1);

    }
}