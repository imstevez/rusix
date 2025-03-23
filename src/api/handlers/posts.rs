use crate::api::response::*;
use crate::datasource::Datasource;
use crate::models::Post;
use crate::repos;
use actix_web::web;
use actix_web::{get, post};
use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
struct ListPostsParams {
    #[serde(default = "default_after")]
    #[validate(range(min = 1))]
    after: i32,
    #[serde(default = "default_size")]
    #[validate(range(min = 1, max = 50))]
    size: i64,
}

fn default_after() -> i32 {
    i32::MAX
}

fn default_size() -> i64 {
    20
}

#[get("")]
async fn list_posts(
    ds: web::Data<Datasource>,
    params: web::Query<ListPostsParams>,
) -> Result<Data<Vec<Post>>, Error> {
    params.validate()?;
    let mut r = ds.rw_db.get().await?;
    let list = repos::list_posts(&mut r, params.after, params.size)
        .await
        .map(|d: Vec<Post>| Data(d))?;
    Ok(list)
}

#[derive(Validate, Deserialize)]
struct CreatePostParams {
    #[validate(length(min = 1, max = 128))]
    title: String,
    #[validate(length(min = 1, max = 1024))]
    body: String,
}

#[post("")]
async fn create_post(
    ds: web::Data<Datasource>,
    params: web::Json<CreatePostParams>,
) -> Result<Data<Post>, Error> {
    params.validate()?;
    let mut r = ds.rw_db.get().await?;
    let item = repos::create_post(&mut r, &params.title, &params.body)
        .await
        .map(|d: Post| Data(d))?;
    Ok(item)
}
