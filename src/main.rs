use std::collections::HashMap;
use std::sync::Mutex;

use nanoid::nanoid;
use rocket::http::uri::Absolute;
use rocket::response::{content, status, Redirect};
use rocket::serde::json::Json;
use rocket::{Rocket, State};
struct Database {
  short_to_long: Mutex<HashMap<String, String>>,
  long_to_short: Mutex<HashMap<String, String>>,
}

use rocket::serde::Deserialize;
#[derive(Deserialize)]
struct UrlToShorten {
  url: String,
}

#[rocket::post("/create-shortened-url", format = "json", data = "<url_to_shorten>")]
async fn create_shortened_url(
  database: &State<Database>,
  url_to_shorten: Json<UrlToShorten>,
) -> Result<status::Created<content::Json<String>>, status::BadRequest<content::Json<String>>> {
  let absolute = Absolute::parse_owned(url_to_shorten.url.clone());
  match absolute {
    Ok(mut long) => {
      long.clear_query();
      long.normalize();
      let is_valid_scheme = long.scheme() == "http" || long.scheme() == "https";
      if long.authority() == None || !is_valid_scheme {
        return Err(status::BadRequest(Some(content::Json(
          "{\"error\": \"could not parse the provided url\"}".to_string(),
        ))));
      }
      let short = nanoid!(10);
      let mut short_to_long = database.short_to_long.lock().unwrap();
      short_to_long.insert(short.clone(), long.clone().to_string());
      let mut long_to_short = database.long_to_short.lock().unwrap();
      long_to_short.insert(long.clone().to_string(), short.clone());
      Ok(
        status::Created::new(format!("http://localhost:8080/{}", short).to_string()).body(
          content::Json(format!("{{\"short\": \"{}\"}}", short.to_owned())),
        ),
      )
    }
    Err(_error) => Err(status::BadRequest(Some(content::Json(
      "{\"error\": \"could not parse the provided url\"}".to_string(),
    )))),
  }
}

#[rocket::post("/get-shortened-url", format = "json", data = "<url_to_shorten>")]
async fn get_shortened_url(
  database: &State<Database>,
  url_to_shorten: Json<UrlToShorten>,
) -> Result<content::Json<String>, status::BadRequest<content::Json<String>>> {
  let absolute = Absolute::parse_owned(url_to_shorten.url.clone());
  match absolute {
    Ok(mut long) => {
      long.clear_query();
      let long_to_short = database.long_to_short.lock().unwrap();
      let short_option = long_to_short.get(&long.to_string());
      match short_option {
        Some(short) => Ok(content::Json(format!(
          "{{\"short\": \"{}\"}}",
          short.to_owned()
        ))),
        None => Err(status::BadRequest(Some(content::Json(
          "{\"error\": \"could not find url\"}".to_string(),
        )))),
      }
    }
    Err(error) => Err(status::BadRequest(Some(content::Json(format!(
      "{{\"error\": \"{}\"}}",
      error.to_string()
    ))))),
  }
}

#[rocket::get("/<short>")]
async fn get_full_url(
  short: &str,
  database: &State<Database>,
) -> Result<Redirect, status::NotFound<content::Json<String>>> {
  let short_to_long = database.short_to_long.lock().unwrap();
  match short_to_long.get(short) {
    Some(long) => Ok(Redirect::to(long.clone())),
    _ => Err(status::NotFound(content::Json(
      "{\"error\": \"could not find url\"}".to_string(),
    ))),
  }
}

#[rocket::main]
async fn main() {
  let database = Database {
    short_to_long: Mutex::new(HashMap::new()),
    long_to_short: Mutex::new(HashMap::new()),
  };
  Rocket::build()
    .manage(database)
    .mount(
      "/api/v1/",
      rocket::routes![create_shortened_url, get_shortened_url],
    )
    .mount("/", rocket::routes![get_full_url])
    .launch()
    .await
    .expect("server to launch");
}
