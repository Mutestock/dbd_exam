
use std::time::SystemTime;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use serde_derive::{Deserialize, Serialize};

use crate::schema::locations;
use crate::schema::locations::dsl;
use crate::schema::locations::dsl::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, PartialEq)]
#[table_name = "locations"]
pub struct Location {
    id: i32,
    street_name: String,
    zipcode: String,
    country: String,
}


#[derive(Insertable, Deserialize, AsChangeset, PartialEq)]
#[table_name = "locations"]
pub struct NewLocation {
    street_name: String,
    zipcode: String,
    country: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct LocationsList(pub Vec<Location>);


impl Location{
    pub fn find(location_id: &i32, connection: &PgConnection) -> Result<Location, diesel::result::Error>{
        locations::table.find(location_id).first(connection)
    }

    pub fn delete(location_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::delete(dsl::locations.find(location_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(location_id: &i32, new_location: &NewLocation, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(dsl::locations.find(location_id))
            .set(new_location)
            .execute(connection)?;
        Ok(())
    }
}


impl NewLocation {
    pub fn create(&self, connection: &PgConnection) -> Result<Location, diesel::result::Error> {
        diesel::insert_into(locations::table)
            .values(self)
            .get_result(connection)
    }
}

impl LocationsList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = locations
            .limit(10)
            .load::<Location>(connection)
            .expect("Error loading locations");
        LocationsList(result)
    }
}
