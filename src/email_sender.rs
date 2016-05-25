/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use lettre::email::EmailBuilder;
use lettre::transport::smtp::{ SecurityLevel, SmtpTransport,
                               SmtpTransportBuilder };
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::smtp::SUBMISSION_PORT;
use lettre::transport::EmailTransport;

pub struct EmailSender {
    connection: SmtpTransport,
    from: String
}

impl EmailSender {
    pub fn new(server: &str, username: &str, password: &str,
               from: Option<String>) -> Result<EmailSender, ()> {
            let builder = match SmtpTransportBuilder::new(
                (server, SUBMISSION_PORT)
            ) {
                Ok(builder) => builder,
                Err(error) => {
                    error!("{:?}", error);
                    return Err(())
                }
            };

            let connection = builder.hello_name("localhost")
                .credentials(username, password)
                .security_level(SecurityLevel::AlwaysEncrypt)
                .smtp_utf8(true)
                .authentication_mechanisms(vec![Mechanism::Plain])
                .connection_reuse(true).build();

            let from = match from {
                Some(from) => from,
                None => String::from(username)
            };

            Ok(EmailSender {
                connection: connection,
                from: from
            })
    }

    pub fn send(&mut self, to: &str, body: &str, subject: &str)
        -> Result<(), ()> {
        let email = match EmailBuilder::new()
            .to(to)
            .from(&*self.from)
            .body(body)
            .subject(subject)
            .build() {
            Ok(email) => email,
            Err(error) => {
                error!("{:?}", error);
                return Err(())
            }
        };

        match self.connection.send(email.clone()) {
            Ok(_) => Ok(()),
            Err(error) => {
                error!("{:?}", error);
                Err(())
            }
        }
    }
}
