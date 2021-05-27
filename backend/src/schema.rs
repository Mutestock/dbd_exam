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
        university_id -> Nullable<Int4>,
        locations_id -> Nullable<Int4>,
    }
}

table! {
    universities (id) {
        id -> Int4,
        country_index -> Varchar,
        university_name -> Varchar,
        website_url -> Varchar,
        locations_id -> Nullable<Int4>,
    }
}

joinable!(people -> locations (locations_id));
joinable!(people -> universities (university_id));
joinable!(universities -> locations (locations_id));

allow_tables_to_appear_in_same_query!(
    locations,
    people,
    universities,
);
