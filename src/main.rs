#![allow(dead_code, unused_variables)]

enum Status {
    Connected,
    Interrupted
}

struct Credentials {
    username: String,
    password: String,
}

fn connect_to_database() -> Status {
    return Status::Connected
}

fn login(creds: Credentials) {
    // Authenticate...
    get_user();
}

fn logout() {
    // log user out...
}

fn get_user() {
    // Get user from database
}

fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds)
    }
}

fn main() {
    println!("Hello, world!");
}
