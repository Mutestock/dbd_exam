table! {
    locations (id) {
        id -> Int4,
        street_name -> Varchar,
        zipcode -> Varchar,
        country -> Varchar,
    }
}

table! {
    people (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        phone_number -> Varchar,
        avatar -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    universities (id) {
        id -> Int4,
        country_index -> Varchar,
        university_name -> Varchar,
        website_url -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    locations,
    people,
    universities,
);
