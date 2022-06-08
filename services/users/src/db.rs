use crate::{
    handlers::models::Filter,
    models::{InsertableUser, User},
    schema::users,
    Connection,
};
use common::models::Pager;
use diesel::{insert_into, pg::Pg, prelude::*, result::Error};

type DbResponse<T> = Result<T, Error>;

pub fn list_users<'a>(pager: Pager, filter: Filter, conn: &Connection) -> DbResponse<Vec<User>> {
    let mut query: users::BoxedQuery<'a, Pg> = users::table.into_boxed();

    if let Some(s) = filter.email {
        query = query.filter(users::email.eq(s));
    }

    if let Some(s) = filter.first_name {
        query = query.filter(users::first_name.like(s));
    }

    if let Some(s) = filter.last_name {
        query = query.filter(users::last_name.like(s));
    }

    if let Some(s) = filter.username {
        query = query.filter(users::username.like(s));
    }

    if let Some(num) = pager.limit {
        query = query.limit(num);
    }

    if let Some(num) = pager.skip {
        query = query.offset(num);
    }

    Ok(query.load::<User>(conn)?)
}

pub fn add_user(item: &InsertableUser, conn: &Connection) -> DbResponse<User> {
    Ok(insert_into(users::table)
        .values(item)
        .get_result::<User>(conn)?)
}

pub fn get_user(id: i32, conn: &Connection) -> DbResponse<User> {
    Ok(users::dsl::users.find(id).first::<User>(conn)?)
}
