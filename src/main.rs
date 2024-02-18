use axum::{
    http::StatusCode,
    routing::{get,post},
    Router,
    response::IntoResponse
};

use std::collections::HashMap;
use time::Date;
use time::macros::date;
use uuid::Uuid;

pub struct Person{
    id: Uuid,
    name: String,
    nick: String,
    birth_date: Date,
    stack: Vec<String>,
}

#[tokio::main]
async fn main() {

    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person = Person{
        id: Uuid::now_v7(),
        name: String::from("Luid"),
        nick: String::from("luidooo"),
        birth_date: date!(2004 - 06 - 17),
        stack: vec!["C".to_string(), "C++".to_string()], 
    }; 
    
    people.insert(person.id, person); 

    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_people));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn search_people() -> impl IntoResponse {
    return (StatusCode::NOT_FOUND, "Busca Pessoas")
}

async fn find_person() -> impl IntoResponse {
    return (StatusCode::NOT_FOUND, "Find")
}

async fn create_person() -> impl IntoResponse {
    return (StatusCode::NOT_FOUND, "Create")
}

async fn count_people() -> impl IntoResponse {
    return (StatusCode::NOT_FOUND, "Count")
}
