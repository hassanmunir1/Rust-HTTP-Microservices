
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

struct AppState {
    users: Arc<Mutex<Vec<User>>>,
}

async fn get_user(user_id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let user_id = user_id.into_inner();
    let users = data.users.lock().unwrap();

    if let Some(user) = users.iter().find(|u| u.id == user_id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

async fn create_user(user: web::Json<User>, data: web::Data<AppState>) -> impl Responder {
    let new_user = user.into_inner();
    let mut users = data.users.lock().unwrap();
    users.push(new_user.clone());

    HttpResponse::Created().json(new_user)
}

async fn update_user(user_id: web::Path<u32>, updated_user: web::Json<User>, data: web::Data<AppState>) -> impl Responder {
    let user_id = user_id.into_inner();
    let mut users = data.users.lock().unwrap();

    if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
        *user = updated_user.into_inner();
        HttpResponse::Ok().json(user.clone())
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

async fn delete_user(user_id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let user_id = user_id.into_inner();
    let mut users = data.users.lock().unwrap();

    if let Some(index) = users.iter().position(|u| u.id == user_id) {
        users.remove(index);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Arc::new(Mutex::new(vec![
            User { id: 1, name: String::from("Hassan") },
            User { id: 2, name: String::from("Hanzala") },
        ])),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/users/{id}").route(web::get().to(get_user)))
            .service(web::resource("/users").route(web::post().to(create_user)))
            .service(web::resource("/users/{id}").route(web::put().to(update_user)))
            .service(web::resource("/users/{id}").route(web::delete().to(delete_user)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
