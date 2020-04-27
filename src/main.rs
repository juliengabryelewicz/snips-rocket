#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate serde_json;
extern crate snips_nlu_lib;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;
use snips_nlu_lib::SnipsNluEngine;
use std::sync::Mutex;

type Engine = Mutex<SnipsNluEngine>;

#[derive(Serialize, Deserialize)]
struct Message {
    content: String
}


fn init_snips() -> SnipsNluEngine {
    let snips_dir = "<folder-of-your-choice>/dataset.model";
    let engine = SnipsNluEngine::from_path(snips_dir).unwrap();
    engine
}

#[catch(404)]
fn not_found(_: &rocket::Request<'_>) -> content::Json<&'static str> {
    content::Json("{'error': '404', 'message': 'Page not found'}")
}

#[post("/parse", format = "json", data = "<message>")]
fn parse(message: Json<Message>, engine: State<Engine>) -> String {
    let query = message.0.content;
    let engine = engine.lock().unwrap();
    let result = engine
    .parse_with_alternatives(
        query.trim(),
        None,
        None,
        0,
        0,
    )
    .unwrap();
    let result_json = serde_json::to_string_pretty(&result).unwrap();
    result_json
}

#[get("/")]
fn index() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}

fn main() {
	let engine = init_snips();
    rocket::ignite().manage(Mutex::new(engine))
    	.mount("/", routes![index,parse])
    	.register(catchers![not_found])
    	.launch();
}