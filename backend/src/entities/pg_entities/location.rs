use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use serde_derive::{Deserialize, Serialize};

use crate::entities::shared_behaviour::CacheAble;
use crate::schema::locations;
use crate::schema::locations::dsl;
use crate::schema::locations::dsl::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, PartialEq, Clone)]
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
pub struct CachedLocation {
    street_name: String,
    zipcode: String,
    country: String,
}

impl CacheAble<Location> for CachedLocation {
    fn from_base(location: Location) -> Self {
        Self {
            street_name: location.street_name,
            zipcode: location.zipcode,
            country: location.country,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationsList(pub Vec<Location>);

#[derive(Serialize, Deserialize)]
pub struct CachedLocationsList(pub Vec<CachedLocation>);

impl Location {
    fn find_non_cached_location(
        location_id: &i32,
        connection: &PgConnection,
    ) -> Result<Location, diesel::result::Error> {
        locations::table.find(location_id).first(connection)
    }

    pub fn find(
        location_id: &i32,
        connection: &PgConnection,
    ) -> Result<CachedLocation, diesel::result::Error> {
        Ok(CachedLocation::from_base(
            Self::find_non_cached_location(location_id, connection)?,
        ))
    }

    pub fn delete(
        location_id: &i32,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::locations.find(location_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(
        location_id: &i32,
        new_location: &NewLocation,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
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

impl CachedLocationsList {
    pub fn list(connection: &PgConnection) -> Self {
        let mut cached_locations = vec![];
        for location in LocationsList::list(connection).0 {
            let stuff = CachedLocation::from_base(location.clone());
            cached_locations.push(stuff);
        }
        Self(cached_locations)
    }

    pub fn deserialize(cache_string: &str) -> Self {
        let res: Self = serde_json::from_str(cache_string)
            .expect("Could not deserialize string into CachedUniversitiesList");
        res
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Could not serialize CachedUniversitiesList")
    }
}
