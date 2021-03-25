use crate::{
    app::db::Connection,
    schema::links::{self, dsl::*},
};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable)]
pub struct Link {
    pub id: i64,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub expired_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct LinkDTO {
    pub url: String,
    pub created_at: NaiveDateTime,
    pub expired_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct LongLinkDTO {
    pub url: String,
    pub keep_sec: Option<i64>,
}

impl Link {
    pub fn find_by_id(i: i64, conn: &Connection) -> QueryResult<Link> {
        links.find(i).get_result::<Link>(conn)
    }

    pub fn insert(new_link: LongLinkDTO, conn: &Connection) -> QueryResult<Link> {
        let created = Utc::now().naive_utc();
        let expired = match new_link.keep_sec {
            Some(value) => Some(created + Duration::seconds(value)),
            None => None,
        };

        let link = LinkDTO {
            url: new_link.url,
            created_at: created,
            expired_at: expired,
        };

        diesel::insert_into(links::table)
            .values(&link)
            .get_result::<Link>(conn)
    }
}
