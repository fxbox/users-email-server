/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// Simple server that allow clients to send user management related emails.
///
/// Only one endpoint is available so far:
///
/// POST /invitation
///
/// This endpoint allows the client to send a invitation to a user to allow
/// her to activate her account. The body must contain a URL from a domain
/// that must be known to the server (i.e. https://<hash>.knilxof.org/<path>)
///
/// In the future, we may want to add new endpoints for extra functionality
/// such as password reset emails, for instance.

extern crate docopt;
extern crate env_logger;
extern crate iron;
extern crate iron_cors;
extern crate lettre;
#[macro_use]
extern crate log;
extern crate mount;
extern crate regex;
extern crate router;
extern crate rustc_serialize;

use docopt::Docopt;
use email_sender::EmailSender;
use iron::{ Chain, Iron };
use iron::method::Method;
use iron_cors::CORS;
use mount::Mount;
use std::sync::{ Arc, RwLock };

mod email_sender;
mod errors;
mod routes;

pub static API_VERSION: &'static str = "v1";

const USAGE: &'static str = "
Usage: users-email-server [-h <hostname>] [-p <port>] [-u <regex>] [--email-server <server>] [--email-user <user>] [--email-password <pass>] [--email-from <from>]

Options:
    -h, --host <host>             Optional. Set local hostname.
    -p, --port <port>             Optional. Set port to listen on for http connections.
        --email-server <server>   Email server URL.
        --email-user <user>       Email user.
        --email-password <pass>   Email password.
        --email-from <from>       Optional. This will be the value of the email 'from' field.
    -u, --url-regex <regex>       Regular expression that url comming in request bodies must match.
";

#[derive(RustcDecodable)]
struct Args {
    flag_host: Option<String>,
    flag_port: Option<u16>,
    flag_email_server: String,
    flag_email_user: String,
    flag_email_password: String,
    flag_email_from: Option<String>,
    flag_url_regex: String
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let port = args.flag_port.unwrap_or(4444);
    let host = args.flag_host.unwrap_or("0.0.0.0".to_string());

    let url_regex = args.flag_url_regex;
    if url_regex.is_empty() {
        panic!("Missing URL regex {}", USAGE);
    }

    let url_regex = format!(r"{}", url_regex);

    // Panic if any of these args is missing.
    let email_server = args.flag_email_server;
    let email_user = args.flag_email_user;
    let email_password = args.flag_email_password;

    if email_server.is_empty() ||
       email_user.is_empty() ||
       email_password.is_empty() {
        panic!("Missing email configuration {}", USAGE);
    }

    let email_sender = Arc::new(RwLock::new(EmailSender::new(
        &email_server,
        &email_user,
        &email_password,
        args.flag_email_from
    ).unwrap()));

    let mut mount = Mount::new();
    mount.mount(&format!("/{}/", API_VERSION),
                routes::create(url_regex, email_sender));

    let mut chain = Chain::new(mount);
    let cors = CORS::new(vec![
        (vec![Method::Post], "invitation".to_owned())
    ]);
    chain.link_after(cors);

    let iron = Iron::new(chain);
    info!("Starting server on {}:{}", host, port);
    let addr = format!("{}:{}", host, port);
    iron.http(addr.as_ref() as &str).unwrap();
}
