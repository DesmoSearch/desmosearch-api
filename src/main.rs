#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;
use serde::{Serialize, Deserialize};

#[get("/graphs/<id>")]
fn get_graph(id: &str) -> Option<String> { 
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
      None => { None } //format!("id: {}, name: {}, creator: {}", cursor)
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

#[post("/graphs", data = "<input>")]
fn set_graph(input: &str) -> Option<String> {
  let connection = rusqlite::Connection::open("db").unwrap();
  let mut stmt = connection.prepare("INSERT INTO graphs VALUES (?, ?, ?)").ok()?;
  let graph_data = serde_json::from_str::<GraphData>(input); 
  return match graph_data {
    Ok(graph_data) => {
      stmt.execute([graph_data.id, graph_data.name, graph_data.creator]).ok()?;
      Some("Success!".to_string())
    }
    Err(_error) => {
      Some("Malformed request.".to_string())
    }
  }
}

#[launch]
fn rocket() -> _ {
  let connection = rusqlite::Connection::open("db").unwrap();
  connection.execute("
    CREATE TABLE IF NOT EXISTS graphs (id TEXT, name TEXT, creator TEXT);
    INSERT INTO graphs VALUES ('vhqlyoddoa', 'Desmos Plane', 'Radian628');
  ", []).unwrap();
  // connection.iterate("SELECT * FROM graphs WHERE creator = 'Radian628'", |results| {
  //   for &(column, value) in results.iter() {
  //     println!("{} = {}", column, value.unwrap())
  //   }
  //   true
  // });
  rocket::build().mount("/", routes![get_graph, set_graph])
}