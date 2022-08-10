use crate::users::model::User;

pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn add_user(&mut self, user: User) {
        self.users.push(user)
    }

    pub fn print_all_users(&self) {
        self.users.iter().for_each(|user| println!("{}", user));
    }
}

pub fn create_user_manager() -> UserManager {
    UserManager { users: Vec::new() }
}
