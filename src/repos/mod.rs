use crate::models;
use diesel::PgConnection;
use diesel::prelude::*;

pub fn list_posts(
    conn: &mut PgConnection,
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
}

pub fn create_post(
    conn: &mut PgConnection,
    title: &str,
    body: &str,
) -> Result<models::Post, diesel::result::Error> {
    use crate::schema::posts::table;
    let new_post = models::NewPost { title, body };
    diesel::insert_into(table)
        .values(&new_post)
        .returning(models::Post::as_returning())
        .get_result(conn)
}
