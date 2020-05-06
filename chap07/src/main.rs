
mod api;
mod db;
mod util;


fn main () {

    let user = api::users::User::new("Julian".to_string());
    println!("{:?}", user);


    //db::save_user(user);

    //api::posts::make_post(user, "Message".to_string())
}