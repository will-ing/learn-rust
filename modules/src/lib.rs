#![allow(dead_code, unused_variables)]
struct Credentials {
    username: String,
    password: String,
}

enum Status {
    Connected,
    Interrupted,
}

fn login(creds: Credentials) {
    // authenticate
    user()
}

fn logout() {
    // log user out
}

fn user() {
    // get the user
}

fn connect_to_db() -> Status {
    return Status::Connected;
}

// Takes the a set of credentials
fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_db() {
        login(creds);
    }
}
