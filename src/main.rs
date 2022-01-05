use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, web::Path, web::Query};
use serde_json::json;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::sync::Mutex;

use rusqlite::named_params;

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug, Deserialize)]
pub enum GraphQueryOrdering {
  #[serde(rename = "id")]
  ID, 
  #[serde(rename = "parent_id")]
  ParentID, 
  #[serde(rename = "name")]
  Name, 
  #[serde(rename = "creator")]
  Creator, 
  #[serde(rename = "upload_date")]
  UploadDate 
}

impl ToString for GraphQueryOrdering {
  fn to_string(&self) -> String {
    match self {
      GraphQueryOrdering::ID => "id".to_owned(),
      GraphQueryOrdering::ParentID => "parent_id".to_owned(),
      GraphQueryOrdering::Name => "name".to_owned(),
      GraphQueryOrdering::Creator => "creator".to_owned(),
      GraphQueryOrdering::UploadDate => "upload_date".to_owned()
    }
  }
} 

#[derive(Debug, Deserialize)]
pub struct GraphQueryParams {
  id: Option<String>,
  parent_id: Option<String>,
  name: Option<String>,
  creator: Option<String>,
  upload_date_start: Option<i64>,
  upload_date_end: Option<i64>,
  limit: Option<i64>,
  offset: Option<i64>,
  sort: Option<GraphQueryOrdering>
}

#[get("/graphs")]
async fn get_graph(state: web::Data<DesmoSearchAPIState>, info: actix_web::web::Query<GraphQueryParams>) -> impl Responder {
  let connection = state.db_connection.lock().unwrap();//rusqlite::Connection::open("db").unwrap();
  let mut stmt = connection.prepare(&format!("SELECT * FROM graphs WHERE 
  id LIKE :id 
  AND parent_id LIKE :parent_id 
  AND name LIKE :name 
  AND creator LIKE :creator 
  AND upload_date BETWEEN :upload_date_start AND :upload_date_end
  ORDER BY {sort}
  LIMIT :limit OFFSET :offset", sort=info.sort.as_ref().unwrap_or(&GraphQueryOrdering::UploadDate).to_string())[..]).unwrap();
  let rows = stmt.query(
    named_params!{
      ":id": info.id.as_ref().unwrap_or(&"%".to_string()),
      ":parent_id": info.parent_id.as_ref().unwrap_or(&"%".to_string()),
      ":name": info.name.as_ref().unwrap_or(&"%".to_string()),
      ":creator": info.creator.as_ref().unwrap_or(&"%".to_string()),
      ":upload_date_start": info.upload_date_start.as_ref().unwrap_or(&0i64),
      ":upload_date_end": info.upload_date_end.as_ref().unwrap_or(&9223372036854775807),
      ":limit": info.limit.as_ref().unwrap_or(&1i64),
      ":offset": info.offset.as_ref().unwrap_or(&0i64)
    }
  ).map_err(actix_web::error::ErrorInternalServerError)?;
  let graph_data_list_data: Vec<GraphData> = rows.mapped(|row| {
    Ok(GraphData {
      id: row.get::<&str, String>("id").unwrap(),
      parent_id: row.get::<&str, String>("parent_id").unwrap(),
      name: row.get::<&str, String>("name").unwrap(),
      creator: row.get::<&str, String>("creator").unwrap(),
      upload_date: Some(row.get::<&str, i64>("upload_date").unwrap())
    })
  }).map(|gd| {
    gd.map_err(actix_web::error::ErrorInternalServerError)
  }).collect::<Result<Vec<GraphData>,actix_web::error::Error>>()?;
  let graph_data_list = GraphDataList { 
    data: graph_data_list_data
  };
  return Ok::<String, actix_web::error::Error>(serde_json::to_string(&graph_data_list).unwrap())
}

#[derive(Serialize, Deserialize)]
struct GraphData {
  id: String,
  parent_id: String,
  name: String,
  creator: String,
  upload_date: Option<i64>
}

#[derive(Serialize, Deserialize)]
struct GraphDataList {
  data: Vec<GraphData>
}

#[post("/graphs")]
async fn post_graph(state: web::Data<DesmoSearchAPIState>, graph_data_list: web::Json<GraphDataList>) -> actix_web::Result<String> {
  let mut connection = state.db_connection.lock().unwrap();
  let tx = connection.transaction().map_err(actix_web::error::ErrorInternalServerError)?;
  let mut stmt = tx.prepare("INSERT INTO graphs VALUES (?, ?, ?, ?, strftime('%s','now'))").map_err(actix_web::error::ErrorInternalServerError)?;
  for graph_data in &graph_data_list.data {
    let stmt_execution = stmt.execute([&graph_data.id, &graph_data.parent_id, &graph_data.name, &graph_data.creator]);
    match stmt_execution {
      Ok(_) => {}
      Err(_err) => {
        return Err(actix_web::error::ErrorInternalServerError("Database failed to insert values."));
      }
    }
  }
  stmt.finalize().map_err(actix_web::error::ErrorInternalServerError)?;
  tx.commit().map_err(actix_web::error::ErrorInternalServerError)?;
  Ok("Successfully inserted graph data.".to_string())
}

async fn manual_hello(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

struct DesmoSearchAPIState {
  db_connection: Mutex<rusqlite::Connection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let connection = rusqlite::Connection::open("db").unwrap();
  connection.execute("
    CREATE TABLE IF NOT EXISTS graphs (id TEXT, parent_id TEXT, name TEXT, creator TEXT, upload_date INTEGER);
  ", []).unwrap();
  connection.execute("
  INSERT INTO graphs VALUES ('vhqlyoddoa', 'unknown', 'Desmos Plane', 'Radian628', strftime('%s','now'));
  ", []).unwrap();

  HttpServer::new(|| {
    let connection2 = rusqlite::Connection::open("db").unwrap();
    App::new()
      .data(DesmoSearchAPIState {
        db_connection: Mutex::new(connection2)
      })
      .data(actix_web::web::PayloadConfig::new(1 << 25))
      .data(actix_web::web::JsonConfig::default().limit(1024 * 1024 * 32))
      .service(hello)
      .service(get_graph)
      .service(post_graph)
      .route("/", web::get().to(manual_hello))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}