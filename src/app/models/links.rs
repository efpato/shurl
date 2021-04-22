use crate::app::db::Connection;
use crate::schema::links::{self, dsl::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Link {
    pub id: i64,
    pub url: String,
    pub redirect_count: i32,
    pub created_at: NaiveDateTime,
    pub expired_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "links"]
pub struct CreateLink {
    pub url: String,
    pub expired_at: Option<NaiveDateTime>,
}

impl Link {
    pub fn list(conn: &Connection) -> QueryResult<Vec<Link>> {
        links.load(conn)
    }

    pub fn find_by_id(link_id: i64, conn: &Connection) -> QueryResult<Link> {
        links.find(link_id).get_result::<Link>(conn)
    }

    pub fn insert(new_link: CreateLink, conn: &Connection) -> QueryResult<Link> {
        diesel::insert_into(links::table)
            .values(&new_link)
            .get_result::<Link>(conn)
    }

    pub fn update(link_id: i64, conn: &Connection) -> QueryResult<Link> {
        diesel::update(links.filter(id.eq(link_id)))
            .set(redirect_count.eq(redirect_count + 1))
            .get_result::<Link>(conn)
    }

    pub fn delete(link_id: i64, conn: &Connection) -> Result<usize, diesel::result::Error> {
        let count = diesel::delete(links.filter(id.eq(link_id))).execute(conn)?;
        Ok(count)
    }
}
