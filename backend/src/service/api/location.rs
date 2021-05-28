#[macro_export]
macro_rules! list_locations {
    () => {
        location_routes::list()
            .and_then(location_handler::list)
    };
}

#[macro_export]
macro_rules! get_location {
    () => {
        location_routes::get()
            .and_then(location_handler::get)
    };
}

#[macro_export]
macro_rules! create_location {
    () => {
        location_routes::create()
            .and_then(location_handler::create)
    };
}


#[macro_export]
macro_rules! update_location {
    () => {
        location_routes::update()
            .and_then(location_handler::update)
    };
}


#[macro_export]
macro_rules! delete_location {
    () => {
        location_routes::delete()
            .and_then(location_handler::delete)
    };
}
