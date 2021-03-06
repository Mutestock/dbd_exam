use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use meilisearch_sdk::document::Document;
use meilisearch_sdk::search::Query;
use meilisearch_sdk::search::SearchResults;
use meilisearch_sdk::client::*;
use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::entities::shared_behaviour::CacheAble;
use crate::schema::people;
use crate::schema::people::dsl;
use crate::schema::people::dsl::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, PartialEq, Clone)]
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
    persons_id: Option<i32>,
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
    locations_id: Option<i32>,
    university_id: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CachedPerson {
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeopleList(pub Vec<Person>);

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedPeopleList(pub Vec<CachedPerson>);

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchPerson {
    person_id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    avatar: String,
}

impl Document for SearchPerson {
    type UIDType = i32;
    fn get_uid(&self) -> &Self::UIDType {
        &self.person_id
    }
}

impl SearchPerson {
    pub async fn search(
        search_str: &str,
        connection: &Client<'_>,
    ) -> Result<SearchResults<Self>, meilisearch_sdk::errors::Error> {
        let people_index = connection
            .get_or_create("people")
            .await
            .expect("Could not get or create people index");

        let query: Query = Query::new(&people_index).with_query(&search_str).build();

        people_index.execute_query(&query).await
    }
}

impl CacheAble<Person> for CachedPerson {
    fn from_base(person: Person) -> Self {
        Self {
            first_name: person.first_name,
            last_name: person.last_name,
            email: person.email,
            phone_number: person.phone_number,
            avatar: person.avatar,
        }
    }
}

impl Person {
    fn find_non_cached_person(
        person_id: &i32,
        connection: &PgConnection,
    ) -> Result<Person, diesel::result::Error> {
        people::table.find(person_id).first(connection)
    }

    pub fn find(
        person_id: &i32,
        connection: &PgConnection,
    ) -> Result<CachedPerson, diesel::result::Error> {
        Ok(CachedPerson::from_base(Self::find_non_cached_person(
            person_id, connection,
        )?))
    }

    pub fn delete(person_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::people.find(person_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(
        person_id: &i32,
        new_person: &NewPerson,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
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

impl CachedPeopleList {
    pub fn list(connection: &PgConnection) -> Self {
        let mut cached_people = vec![];
        for person in PeopleList::list(connection).0 {
            let stuff = CachedPerson::from_base(person.clone());
            cached_people.push(stuff);
        }
        Self(cached_people)
    }

    pub fn deserialize(cache_string: &str) -> Self {
        let res: Self = serde_json::from_str(cache_string)
            .expect("Could not deserialize string into CachedPeopleList");
        res
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Could not serialize CachedPeopleList")
    }
}
