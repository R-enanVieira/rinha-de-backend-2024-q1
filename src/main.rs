use std::{collections::HashMap, env, net::SocketAddr, sync::Arc};

use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get,post},
    Router,
};

use tokio::sync::RwLock;
use serde::Serialize;
use serde::Deserialize;
use time::Date;
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Clone, Serialize)]
pub struct Person{
    pub id: Uuid,
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
pub struct NewPerson{
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>,
}


type AppState = Arc<RwLock<HashMap<Uuid, Person>>>;

#[tokio::main]
async fn main() {

    let port = env::var("PORT")
        .ok()
        .and_then(|port|port.parse::<u16>().ok())
        .unwrap_or(9999);

    let people: HashMap<Uuid, Person> = HashMap::new();
    let app_state = Arc::new(RwLock::new(people));

    /*
    let person = Person{
        id: Uuid::now_v7(),
        name: String::from("jao"),
        nick: String::from("jao123"),
        birth_date: date!(2003 - 02 - 27),
        stack: vec!["javascript".to_string(), "angular".to_string()].into(),
    };
    people.insert(person.id, person);
    */

    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_people))
        .with_state(app_state);

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("listening on {}", addr);
        axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn search_people() -> impl IntoResponse {
    return (StatusCode::NOT_FOUND, "Busca Pessoas")
}

async fn find_person(
    State(people): State<AppState>,
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {
    match people.read().await.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_person(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>,
) -> impl IntoResponse {
    if new_person.name.len() > 100 || new_person.nick.len() > 32 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    match new_person.stack {
        Some(ref stack) => {
            if stack.iter().any(|tech| tech.len() > 32)
               return Err(StatusCode::UNPROCESSABLE_ENTITY)
        }
        None => {}
    };

    let id = Uuid::now_v7();
    let person = Person {
        id,
        name: new_person.name,
        birth_date: new_person.birth_date,
        nick: new_person.nick,
        stack: new_person.stack,
    };

    people.write().await.insert(id, person.clone());
    
    Ok((StatusCode::CREATED, Json(person))) 
}

async fn count_people(State(people): State<AppState>) -> impl IntoResponse {
    Json(people.read().await.len()) 
}
