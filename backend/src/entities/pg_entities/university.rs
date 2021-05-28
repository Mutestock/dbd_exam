use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use serde_derive::{Deserialize, Serialize};

use crate::schema::universities;
use crate::schema::universities::dsl;
use crate::schema::universities::dsl::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[table_name = "universities"]
pub struct University {
    id: i32,
    university_name: String,
    country_index: String,
    website_url: String,
    locations_id: Option<i32>,
}

#[derive(Insertable, Deserialize, AsChangeset, PartialEq)]
#[table_name = "universities"]
pub struct NewUniversity {
    university_name: String,
    country_index: String,
    website_url: String,
    locations_id: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CachedUniversity {
    university_name: String,
    country_index: String,
    website_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UniversitiesList(pub Vec<University>);

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedUniversitiesList(pub Vec<CachedUniversity>);

impl CachedUniversity {
    pub fn deserialize(cache_string: &str) -> Self {
        let res: Self = serde_json::from_str(cache_string)
            .expect("Could not deserialize string into CashedUniversity");
        res
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Could not serialize CachedUniversity")
    }

    pub fn from_university(university: University) -> Self {
        Self {
            university_name: university.university_name,
            country_index: university.country_index,
            website_url: university.website_url,
        }
    }
}

impl University {
    fn find_non_cached_university(
        university_id: &i32,
        connection: &PgConnection,
    ) -> Result<University, diesel::result::Error> {
        universities::table.find(university_id).first(connection)
    }

    pub fn find(
        university_id: &i32,
        connection: &PgConnection,
    ) -> Result<CachedUniversity, diesel::result::Error> {
        Ok(CachedUniversity::from_university(
            Self::find_non_cached_university(university_id, connection)?,
        ))
    }

    pub fn delete(
        university_id: &i32,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::universities.find(university_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(
        university_id: &i32,
        new_university: &NewUniversity,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
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
            .expect("Error loading universities");
        UniversitiesList(result)
    }
}

impl CachedUniversitiesList {
    pub fn list(connection: &PgConnection) -> Self {
        let mut cached_universities = vec![];
        for university in UniversitiesList::list(connection).0 {
            let stuff = CachedUniversity::from_university(university.clone());
            cached_universities.push(stuff);
        }
        Self(cached_universities)
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
