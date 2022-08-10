use crate::users::manager::create_user_manager;

mod users;

fn main() {
    let mut marcus: users::model::User = users::model::create_user(String::from("Marcus"));
    marcus.change_activity_status(false);
    marcus.change_username(String::from("Sparko"));
    marcus.change_activity_status(true);

    let mut user_manager = create_user_manager();
    user_manager.add_user(marcus);

    user_manager.print_all_users();

    let users = user_manager.get_users();
    let uuid_to_find = match users.clone().into_iter().nth(0) {
        Some(user) => user.uuid.to_string(),
        None => panic!("no user found"),
    };
    let found_user = user_manager.find_user_by_uuid(uuid_to_find);

    match found_user {
        Some(user) => println!("I found user: {}", user),
        None => println!("No user found"),
    }
}
