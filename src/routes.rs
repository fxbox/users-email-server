/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use email_sender::EmailSender;
use errors::*;
use iron::prelude::*;
use iron::status::Status;
use regex::Regex;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{ Arc, RwLock };

fn invitation(req: &mut Request, domain_regex: String,
              email_sender: Arc<RwLock<EmailSender>>) -> IronResult<Response> {
    #[derive(RustcDecodable, Debug)]
    struct InvitationBody {
        url:  String,
        email: String
    }

    let mut payload = String::new();
    if let Err(_) = req.body.read_to_string(&mut payload) {
        return EndpointError::with(Status::InternalServerError, 501);
    };
    let body: InvitationBody = match json::decode(&payload) {
        Ok(body) => body,
        Err(error) => {
            error!("{:?}", error);
            return from_decoder_error(error);
        }
    };

    let url = body.url;
    let email = body.email;

    info!("POST /invitation url={} email={} regex={}", url, email, domain_regex);

    let reg = match Regex::new(&domain_regex) {
        Ok(reg) => reg,
        Err(_) => {
            // Malformed regex.
            return EndpointError::with(Status::BadRequest, 101);
        }
    };

    if !reg.is_match(&url) {
        // Invalid url.
        return EndpointError::with(Status::BadRequest, 102);
    }

    let mut sender = match email_sender.write() {
        Ok(sender) => sender,
        Err(_) => {
            error!("Poisoned RwLock email_sender");
            return EndpointError::with(Status::InternalServerError, 501);
        }
    };

    match sender.send(&email, &url, "Welcome to Link") {
        Ok(_) => Ok(Response::with(Status::Ok)),
        Err(_) => EndpointError::with(Status::InternalServerError, 501)
    }
}

pub fn create(domain_regex: String,
              email_sender: Arc<RwLock<EmailSender>>) -> Router {
    let mut router = Router::new();

    router.post("invitation", move |req: &mut Request| -> IronResult<Response> {
        invitation(req, domain_regex.clone(), email_sender.clone())
    });

    router
}
