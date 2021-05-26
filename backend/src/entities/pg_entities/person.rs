use std::time::SystemTime;


struct Person {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    avatar: String,
    created_at: Option<SystemTime>,
    updated_at: Option<SystemTime>,
}

struct NewPerson {
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    avatar: String,
}



