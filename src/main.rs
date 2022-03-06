#![allow(dead_code, unused_variables)]

mod database {
    pub enum Status {
        Connected,
        Interrupted
    }

    pub fn get_user() {
        // Get user from database
    }

    pub fn connect_to_database() -> Status {
        return Status::Connected
    }
}

mod auth_utils {
    pub fn login(creds: models::Credentials) {
        // Authenticate...
        crate::database::get_user();
    }

    pub fn logout() {
        // log user out...
    }

    // You can create helpers within a module that are 
    // not exposed to public
    fn private_helper() {
        // Private helper function
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds)
    }
}

fn main() {
    println!("Hello, world!");
}
