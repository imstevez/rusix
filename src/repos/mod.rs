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
