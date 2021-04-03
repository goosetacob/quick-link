#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use rocket::response::status::BadRequest;
use rocket_contrib::json::Json;
use url::Url;

mod open_graph;

#[macro_use]
extern crate rocket;

struct Link {
    url: Url,
}

impl<'r> FromFormValue<'r> for Link {
    type Error = BadRequest<String>;

    fn from_form_value(value: &'r RawStr) -> Result<Self, Self::Error> {
        let decoded_value = match value.url_decode() {
            Ok(dvalue) => dvalue,
            Err(_) => return Err(BadRequest(Some(String::from("failed decode url")))),
        };
        println!("-----{}", decoded_value);
        match Url::parse(&decoded_value) {
            Ok(url) => Ok(Self { url }),
            Err(_) => Err(BadRequest(Some(String::from("failed to parse url")))),
        }
    }
}

#[get("/opengraph?<link>")]
fn opengraph(link: Link) -> Result<Json<open_graph::LinkPreview>, BadRequest<String>> {
    match open_graph::collect_data(&link.url) {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(BadRequest(Some(e.to_string()))),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![opengraph]).launch();
}
