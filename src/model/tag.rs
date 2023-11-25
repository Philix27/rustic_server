use crate::config::db;
use crate::schema::tags;
use actix_web::http::header::Date;
use diesel::prelude::*;
use diesel::sql_types::{Nullable, Timestamp};

trait CrudOperations<'a> {
    fn create(title: &'a str, subtitle: &'a str);
    fn read(title: &'a str, subtitle: &'a str);
    fn update(title: &'a str, subtitle: &'a str);
    fn delete(title: &'a str, subtitle: &'a str);
}
#[derive(Queryable, Insertable, Selectable, Debug, PartialEq)]
#[diesel(table_name = tags)]
pub struct Tag<'a> {
    // pub id: i32,
    pub title: &'a str,
    pub subtitle: &'a str,
    // pub created_at: Nullable<Timestamp>,
    // pub updated_at: Nullable<Timestamp>,
}
pub struct NewTag<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
}

impl<'a> CrudOperations<'a> for Tag<'a> {
    fn create(d_title: &'a str, d_subtitle: &'a str) {
        use crate::schema::tags::dsl::tags;
        let mut conn = db::setup_connection();

        let data_value = Tag {
            title: d_title,
            subtitle: d_subtitle,
        };

        diesel::insert_into(tags)
            .values(&data_value)
            .execute(&mut conn)
            .expect("Error creating video");
    }

    fn read(d_title: &'a str, d_subtitle: &'a str) {
        use crate::schema::tags::dsl::tags;
        let mut conn = db::setup_connection();

        let data_value = Tag {
            title: d_title,
            subtitle: d_subtitle,
        };

        diesel::insert_into(tags)
            .values(&data_value)
            .execute(&mut conn)
            .expect("Error creating video");
    }

    fn update(d_title: &'a str, d_subtitle: &'a str) {
        use crate::schema::tags::dsl::tags;
        let mut conn = db::setup_connection();

        let data_value = Tag {
            title: d_title,
            subtitle: d_subtitle,
        };

        diesel::insert_into(tags)
            .values(&data_value)
            .execute(&mut conn)
            .expect("Error creating video");
    }

    fn delete(d_title: &'a str, d_subtitle: &'a str) {
        use crate::schema::tags::dsl::tags;
        let mut conn = db::setup_connection();

        let data_value = Tag {
            title: d_title,
            subtitle: d_subtitle,
        };

        diesel::insert_into(tags)
            .values(&data_value)
            .execute(&mut conn)
            .expect("Error creating video");
    }
}
