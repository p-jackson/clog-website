#[macro_use]
extern crate nickel;
extern crate clog;
extern crate rustc_serialize;
extern crate uuid;

use nickel::{Nickel, JsonBody, HttpRouter, StaticFilesHandler};
use clog_config::ClogConfig;
use clog_result::ClogResult;
use rustc_serialize::json;
use std::error::Error;
use uuid::Uuid;

mod git;
mod clog_interact;
mod clog_config;
mod clog_result;

fn main() {
    let mut server = Nickel::new();

    server.post("/generate",
                middleware! { |request, response|

        let clog_config = request.json_as::<ClogConfig>().unwrap();
        let repo_name = Uuid::new_v4().to_string();

        let result = if let Err(err) = git::clone(&clog_config.repository, &repo_name) {
            ClogResult {
                changelog: "".to_owned(),
                error: err.description().to_owned(),
            }
        } else {
            let changelog = clog_interact::generate_changelog(&repo_name, &clog_config.repository);

            match changelog {
                Ok(c) => ClogResult {
                    changelog: c,
                    error: "".to_owned()
                },
                Err(e) => ClogResult {
                    changelog: "".to_owned(),
                    error: format!("{}", e)
                }
            }

        };

        json::encode(&result).unwrap()
    });

    server.utilize(StaticFilesHandler::new("public/build"));

    server.listen(("0.0.0.0", get_server_port()));
}

fn get_server_port() -> u16 {
    std::env::var("PORT").unwrap_or("6767".to_string()).parse().unwrap()
}
