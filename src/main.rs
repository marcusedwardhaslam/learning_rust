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
}
