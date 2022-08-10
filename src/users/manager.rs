use crate::users::model::User;

pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    /**
     * Adds a user to the user manager's user vector
     */
    pub fn add_user(&mut self, user: User) {
        self.users.push(user)
    }

    /**
     * Attempts to find a user from the user manager's user vector
     * returns an option with the user or none
     */
    pub fn find_user_by_uuid(&self, user_uuid_as_string: String) -> Option<&User> {
        self.users
            .iter()
            .find(|user| user.uuid.to_string() == user_uuid_as_string)
    }

    /**
     * Prints all users in the user manager's user vector
     */
    pub fn print_all_users(&self) {
        self.users
            .iter()
            .clone()
            .for_each(|user| println!("{}", user));
    }

    /**
     * Returns a reference to the user manager's vector of users
     */
    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }
}

/**
 * Creates a new user manager instance
 */
pub fn create_user_manager() -> UserManager {
    UserManager { users: Vec::new() }
}
