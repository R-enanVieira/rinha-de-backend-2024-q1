use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get,post},
    Router,
};

use tokio::sync::Mutex;
use serde::Serialize;
use serde::Deserialize;
use time::{macros::date, Date};
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[year] - [month] - [day]");

#[derive(Clone, Serialize)]
pub struct Person{
    id: Uuid,
    #[serde(rename = "nome")]
    name: String,
    #[serde(rename = "apelido")]
    nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    birth_date: Date,
    stack: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
pub struct NewPerson{
    #[serde(rename = "nome")]
    name: String,
    #[serde(rename = "apelido")]
    nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    birth_date: Date,
    stack: Option<Vec<String>>,
}


type AppState = Arc<Mutex<HashMap<Uuid, Person>>>;

#[tokio::main]
async fn main() {

    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person = Person{
        id: Uuid::now_v7(),
        name: String::from("Luid"),
        nick: String::from("luidooo"),
        birth_date: date!(2004 - 06 - 17),
        stack: vec!["C".to_string(), "C++".to_string()].into(),
    };

    //println!("{}", person.id);

    people.insert(person.id, person);

    let AppState = Arc::new(Mutex::new(people));

    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_people))
        .with_state(AppState);

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

async fn find_person(State(people): State<AppState>, Path(person_id): Path<Uuid>) -> impl IntoResponse {
    //let my_people = people.lock().await;
    match people.lock().await.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_person(State(people): State<AppState>, Json(new_person): Json<NewPerson>) -> impl IntoResponse {
    let id = Uuid::now_v7();
    let person = Person {
        id,
        name: new_person.name,
        birth_date: new_person.birth_date,
        nick: new_person.nick,
        stack: new_person.stack,
    };

    people.lock().await.insert(id, person.clone());
    (StatusCode::OK, Json(person))
}

async fn count_people(State(people): State<AppState>) -> impl IntoResponse {
    let count = people.lock().await.len();
    (StatusCode::NOT_FOUND, Json(count))
}
