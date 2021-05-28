#[macro_export]
macro_rules! list_people {
    () => {
        person_routes::list()
            .and_then(person_handler::list)
    };
}

#[macro_export]
macro_rules! get_person {
    () => {
        person_routes::get()
            .and_then(person_handler::get)
    };
}

#[macro_export]
macro_rules! create_person {
    () => {
        person_routes::create()
            .and_then(person_handler::create)
    };
}


#[macro_export]
macro_rules! update_person {
    () => {
        person_routes::update()
            .and_then(person_handler::update)
    };
}


#[macro_export]
macro_rules! delete_person {
    () => {
        person_routes::delete()
            .and_then(person_handler::delete)
    };
}
