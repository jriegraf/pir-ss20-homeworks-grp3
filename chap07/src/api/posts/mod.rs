use super::users::User;

pub fn make_post(u: &User, s: String){
    println!("Made post {} from user {}.", s, u.name)
}