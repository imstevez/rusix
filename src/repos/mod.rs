use crate::models;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn list_posts(
    conn: &mut AsyncPgConnection,
    after: i32,
    size: i64,
) -> Result<Vec<models::Post>, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    posts
        .filter(id.lt(after))
        .order(id.desc())
        .limit(size)
        .select(models::Post::as_select())
        .load(conn)
        .await
}

pub async fn create_post(
    conn: &mut AsyncPgConnection,
    title: &str,
    body: &str,
) -> Result<models::Post, diesel::result::Error> {
    use crate::schema::posts::table;
    let new_post = models::NewPost { title, body };
    diesel::insert_into(table)
        .values(&new_post)
        .returning(models::Post::as_returning())
        .get_result(conn)
        .await
}
