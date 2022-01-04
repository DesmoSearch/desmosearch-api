use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, web::Path, Result};
use serde_json::json;
use serde::{Serialize, Deserialize};
use std::fmt;

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[get("/graphs/{id}")]
async fn get_graph(info: Path<String>) -> impl Responder {
  let id = info.into_inner();
  println!("{}", id);
  let connection = rusqlite::Connection::open("db").unwrap();
  let mut stmt = connection.prepare("SELECT * FROM graphs WHERE id = ?").unwrap();
  let mut rows = stmt.query([id]).ok()?;
  //cursor.bind_by_name(vec![(":id", sqlite::Value::String(id.to_owned()))]).unwrap();
  let next_row = rows.next();
  return match next_row {
    Ok(maybe_row) => match maybe_row { 
      Some(row) => {
        Some(json!(
          {
            "id": row.get::<&str, String>("id").unwrap(),
            "name": row.get::<&str, String>("name").unwrap(),
            "creator": row.get::<&str, String>("creator").unwrap(),
          }
        ).to_string())
       }
      None => { Some("Not found!".to_string()) } //format!("id: {}, name: {}, creator: {}", cursor)
    }
    Err(_error) => { None }
  }
}

#[derive(Serialize, Deserialize)]
struct GraphData {
  id: String,
  name: String,
  creator: String
}

#[derive(Serialize, Deserialize)]
struct GraphDataList {
  data: Vec<GraphData>
}

#[derive(Debug)]
struct DatabaseError {
  reason: String
}
impl DatabaseError {
  fn new(msg: &str) -> DatabaseError {
    DatabaseError{reason: msg.to_string()}
  }
}
impl fmt::Display for DatabaseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.reason)
  }
}
impl std::error::Error for DatabaseError { 
  fn description(&self) -> &str {
    &self.reason
  }
}
impl actix_web::ResponseError for DatabaseError {
  fn error_response(&self) -> actix_web::HttpResponse {
    actix_web::HttpResponse::build(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR).json("Database Error.")
  }
  // fn status_code(&self) -> actix_web::http::StatusCode {
  //   actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
  // }
}

#[post("/graphs")]
async fn post_graph(graph_data_list: web::Json<GraphDataList>) -> Result<String> {
  let connection = rusqlite::Connection::open("db").unwrap();
  let maybe_stmt = connection.prepare("INSERT INTO graphs VALUES (?, ?, ?)");
  match maybe_stmt {
    Ok(mut stmt) => {
      for graph_data in &graph_data_list.data {
        let stmt_execution = stmt.execute([&graph_data.id, &graph_data.name, &graph_data.creator]);
        match stmt_execution {
          Ok(_) => {}
          Err(_err) => {
            return Err(actix_web::error::ErrorInternalServerError("Database failed to insert values."));
          }
        }
      }
      Ok("Successfully inserted graph data.".to_string())
    }
    Err(_err) => {
      Err(actix_web::error::ErrorInternalServerError("Database failed to load query."))
    }
  }
}

async fn manual_hello(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let connection = rusqlite::Connection::open("db").unwrap();
  connection.execute("
    CREATE TABLE IF NOT EXISTS graphs (id TEXT, name TEXT, creator TEXT);
  ", []).unwrap();
  connection.execute("
  INSERT INTO graphs VALUES ('vhqlyoddoa', 'Desmos Plane', 'Radian628');
  ", []).unwrap();

  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(get_graph)
      .service(post_graph)
      .route("/", web::get().to(manual_hello))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}