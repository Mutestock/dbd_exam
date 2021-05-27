use std::time::SystemTime;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use serde_derive::{Deserialize, Serialize};

use crate::schema::people;
use crate::schema::people::dsl;
use crate::schema::people::dsl::*;


#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, PartialEq)]
#[table_name = "people"]
pub struct Person {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    avatar: String,
    created_at: Option<SystemTime>,
    updated_at: Option<SystemTime>,
    locations_id: Option<i32>,
    university_id: Option<i32>,
}

#[derive(Insertable, Deserialize, AsChangeset, PartialEq)]
#[table_name = "people"]
pub struct NewPerson {
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeopleList(pub Vec<Person>);


impl Person{
    pub fn find(person_id: &i32, connection: &PgConnection) -> Result<Person, diesel::result::Error>{
        people::table.find(person_id).first(connection)
    }

    pub fn delete(person_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::delete(dsl::people.find(person_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(person_id: &i32, new_person: &NewPerson, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(dsl::people.find(person_id))
            .set(new_person)
            .execute(connection)?;
        Ok(())
    }
}


impl NewPerson {
    pub fn create(&self, connection: &PgConnection) -> Result<Person, diesel::result::Error> {
        diesel::insert_into(people::table)
            .values(self)
            .get_result(connection)
    }
}

impl PeopleList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = people
            .limit(10)
            .load::<Person>(connection)
            .expect("Error loading persons");
        PeopleList(result)
    }
}
