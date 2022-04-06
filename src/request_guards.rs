use chrono::Local;
use rocket::request::FromRequest;
use rocket::{http::Status, outcome::Outcome};

pub const SESSION_COOKIE_NAME: &str = "sysc4806group13project";

pub mod state {
    use chrono::Local;

    use chrono::DateTime;
    use std::collections::HashMap;

    pub enum SessionType {
        Applicant(i32),
        Professor(i32),
        Administrator(),
    }

    pub type ExpirationTime = DateTime<Local>;
    pub type SessionTokens = HashMap<String, (SessionType, ExpirationTime)>;
}

#[derive(Clone, Copy)]
pub struct Professor {
    pub professor_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Professor {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let result: Result<Professor, ()> = try {
            let session_token_lock = request
                .rocket()
                .state::<crate::SessionTokenState>()
                .ok_or(())?;
            let mut session_tokens = session_token_lock.lock().await;
            let cookies = request.cookies();

            let cust_session_cookie = cookies.get_private(&SESSION_COOKIE_NAME).ok_or(())?;

            let (session_type, expiration_time) =
                session_tokens.get(cust_session_cookie.value()).ok_or(())?;
            if Local::now() > expiration_time.clone() {
                session_tokens.remove(cust_session_cookie.value());
                Err(())?
            } else {
                use state::SessionType;
                match session_type {
                    &SessionType::Professor(professor_id) => Professor { professor_id },
                    &_ => Err(())?,
                }
            }
        };
        match result {
            Ok(professor) => Outcome::Success(professor),
            Err(_) => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}

pub struct Applicant {
    pub applicant_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Applicant {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let result: Result<Applicant, ()> = try {
            let session_token_lock = request
                .rocket()
                .state::<crate::SessionTokenState>()
                .ok_or(())?;
            let mut session_tokens = session_token_lock.lock().await;
            let cookies = request.cookies();

            let session_cookie = cookies.get_private(&SESSION_COOKIE_NAME).ok_or(())?;

            let (session_type, expiration_time) =
                session_tokens.get(session_cookie.value()).ok_or(())?;
            if Local::now() > expiration_time.clone() {
                session_tokens.remove(session_cookie.value());
                Err(())?
            } else {
                use state::SessionType;
                match session_type {
                    &SessionType::Applicant(applicant_id) => Applicant { applicant_id },
                    &_ => Err(())?,
                }
            }
        };
        match result {
            Ok(applicant) => Outcome::Success(applicant),
            Err(_) => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}

pub struct Administrator {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Administrator {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let result: Result<Administrator, ()> = try {
            let session_token_lock = request
                .rocket()
                .state::<crate::SessionTokenState>()
                .ok_or(())?;
            let mut session_tokens = session_token_lock.lock().await;
            let cookies = request.cookies();

            let session_cookie = cookies.get_private(&SESSION_COOKIE_NAME).ok_or(())?;

            let (session_type, expiration_time) =
                session_tokens.get(session_cookie.value()).ok_or(())?;
            if Local::now() > expiration_time.clone() {
                session_tokens.remove(session_cookie.value());
                Err(())?
            } else {
                use state::SessionType;
                match session_type {
                    &SessionType::Administrator() => Administrator {},
                    &_ => Err(())?,
                }
            }
        };
        match result {
            Ok(administrator) => Outcome::Success(administrator),
            Err(_) => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
