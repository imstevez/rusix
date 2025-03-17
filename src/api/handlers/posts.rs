use crate::api::response::Response;
use crate::datasource::Datasource;
use crate::models;
use crate::repos;
use actix_web::web::{Data, Json, Query};
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
    ds: Data<Datasource>,
    params: Query<ListPostsParams>,
) -> Response<Vec<models::Post>> {
    if let Err(err) = params.validate() {
        return Response::params_error(err.to_string());
    }

    let r = ds.rw_db.get();
    if let Err(err) = r {
        return Response::internal_error(err.to_string());
    }

    match repos::list_posts(&mut r.unwrap(), params.after, params.size) {
        Ok(list) => Response::ok(Some(list)),
        Err(err) => Response::internal_error(err.to_string()),
    }
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
    _ds: Data<Datasource>,
    params: Json<CreatePostParams>,
) -> Response<models::Post> {
    println!("{},{}", params.title, params.body);
    Response::ok(None)
}
