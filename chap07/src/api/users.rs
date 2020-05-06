
#[derive(Debug)]
pub struct User {
    pub name: String,
}
impl User {
    pub fn new(name: String) -> User {
        User{name}
    }
}