use std::fmt;
use uuid::Uuid;

pub struct User {
    active: bool,
    username: String,
    pub uuid: Uuid,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User ID: {} - {} is {}",
            self.uuid,
            self.username,
            if self.active { "active" } else { "inactive" }
        )
    }
}

impl User {
    /**
     * Sets the user's activity to the value of the active_status boolean
     */
    pub fn change_activity_status(&mut self, active_status: bool) {
        self.active = active_status;
    }

    /**
     * Sets the user's username to the value of the new_username string
     */
    pub fn change_username(&mut self, new_username: String) {
        self.username = new_username;
    }
}

/**
 * Creates a User in memory and returns ownership
 */
pub fn create_user(username: String) -> User {
    User {
        active: true,
        username: username,
        uuid: Uuid::new_v4(),
    }
}
