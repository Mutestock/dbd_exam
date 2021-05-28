#[macro_export]
macro_rules! list_universities {
    () => {
        university_routes::list()
            .and_then(university_handler::list)
    };
}

#[macro_export]
macro_rules! get_university {
    () => {
        university_routes::get()
            .and_then(university_handler::get)
    };
}

#[macro_export]
macro_rules! create_university {
    () => {
        university_routes::create()
            .and_then(university_handler::create)
    };
}


#[macro_export]
macro_rules! update_university {
    () => {
        university_routes::update()
            .and_then(university_handler::update)
    };
}


#[macro_export]
macro_rules! delete_university {
    () => {
        university_routes::delete()
            .and_then(university_handler::delete)
    };
}
