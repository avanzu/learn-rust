pub mod models;
use self::models::{SignUp, Filter};
use crate::db::{add_user, get_user, list_users};
use crate::{errors::ServiceError, models::InsertableUser, Pool};
use actix_web::{web, HttpResponse, Responder};
use common::models::Pager;
use log::info;

type ServiceResponse<T = HttpResponse> = Result<T, ServiceError>;

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/users")
                .route(web::get().to(list))
                .route(web::post().to(insert)),
        )
        .service(
            web::resource("/users/{id}")
                .route(web::get().to(fetch))
                .route(web::post().to(replace))
                .route(web::patch().to(update))
                .route(web::delete().to(remove)),
        );
}

async fn list(db: web::Data<Pool>, pager: web::Query<Pager>, filter: web::Query<Filter>) -> ServiceResponse {
    let conn = db.get()?;
    let result = web::block(move || list_users(pager.into_inner(),filter.into_inner(), &conn) ).await??;
    Ok(HttpResponse::Ok().json(result))
}

async fn insert(db: web::Data<Pool>, item: web::Json<SignUp>) -> ServiceResponse {
    info!("adding user");
    let conn = db.get()?;
    let user = InsertableUser::from_signup(item.into_inner())?;
    let user = web::block(move || add_user(&user, &conn)).await??;

    Ok(HttpResponse::Created().json(user))
}

async fn fetch(db: web::Data<Pool>, user_id: web::Path<i32>) -> ServiceResponse {
    let conn = db.get()?;
    let user = web::block(move || get_user(user_id.into_inner(), &conn)).await??;

    Ok(HttpResponse::Ok().json(user))
}

async fn replace(user_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello from replace user {}", user_id))
}

async fn update(user_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello from update user {}", user_id))
}

async fn remove(user_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello from delete user {}", user_id))
}
