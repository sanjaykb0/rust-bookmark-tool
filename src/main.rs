#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::Redirect;
mod utils;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {

    let command = utils::get_str_query(&cmd);
    let redirect_url = match command {
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "gh" => utils::github::GH_construct_github_query(&cmd),
        _ => utils::google::construct_google_query(&cmd)
    };

    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}