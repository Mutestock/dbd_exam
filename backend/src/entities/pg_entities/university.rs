
use std::time::SystemTime;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use serde_derive::{Deserialize, Serialize};

use crate::schema::universities;
use crate::schema::universities::dsl;
use crate::schema::universities::dsl::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, PartialEq)]
#[table_name = "universities"]
struct University {
    id: i32,
    university_name: String,
    country_index: String,
    website_url: String,
    locations_id: Option<i32>,
}

#[derive(Insertable, Deserialize, AsChangeset, PartialEq)]
#[table_name = "universities"]
struct NewUniversity {
    university_name: String,
    country_index: String,
    website_url: String,
    locations_id: Option<i32>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UniversitiesList(pub Vec<University>);


impl University{
    pub fn find(university_id: &i32, connection: &PgConnection) -> Result<University, diesel::result::Error>{
        universities::table.find(university_id).first(connection)
    }

    pub fn delete(university_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::delete(dsl::universities.find(university_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(university_id: &i32, new_university: &NewUniversity, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(dsl::universities.find(university_id))
            .set(new_university)
            .execute(connection)?;
        Ok(())
    }
}


impl NewUniversity {
    pub fn create(&self, connection: &PgConnection) -> Result<University, diesel::result::Error> {
        diesel::insert_into(universities::table)
            .values(self)
            .get_result(connection)
    }
}

impl UniversitiesList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = universities
            .limit(10)
            .load::<University>(connection)
            .expect("Error loading universitys");
        UniversitiesList(result)
    }
}
