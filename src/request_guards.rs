use chrono::Local;
use rocket::request::FromRequest;
use rocket::{http::Status, outcome::Outcome};

pub mod state {
    use chrono::Local;

    use chrono::DateTime;
    use serde::Serialize;
    use std::collections::HashMap;

    #[derive(Serialize, Clone, Copy, Debug)]
    pub enum SessionType {
        Applicant(i32),
        Professor(i32),
        Administrator,
    }

    pub type ExpirationTime = DateTime<Local>;
    pub type SessionTokens = HashMap<String, (SessionType, ExpirationTime)>;
}

pub const SESSION_TOKEN_HEADER_NAME: &'static str = "X-Session-Token";

pub struct SessionTokenHeader {
    pub session_token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionTokenHeader {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        match request.headers().get_one(SESSION_TOKEN_HEADER_NAME) {
            Some(v) => Outcome::Success(SessionTokenHeader {
                session_token: v.to_string(),
            }),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
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
            let token = request
                .headers()
                .get_one(SESSION_TOKEN_HEADER_NAME)
                .ok_or(())?;

            let (session_type, expiration_time) = session_tokens.get(token).ok_or(())?;
            if Local::now() > expiration_time.clone() {
                session_tokens.remove(token);
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
            let token = request
                .headers()
                .get_one(SESSION_TOKEN_HEADER_NAME)
                .ok_or(())?;

            let (session_type, expiration_time) = session_tokens.get(token).ok_or(())?;
            if Local::now() > expiration_time.clone() {
                session_tokens.remove(token);
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
            let token = request
                .headers()
                .get_one(SESSION_TOKEN_HEADER_NAME)
                .ok_or(())?;

            let (session_type, expiration_time) = session_tokens.get(token).ok_or(())?;
            if Local::now() > expiration_time.clone() {
                session_tokens.remove(token);
                Err(())?
            } else {
                use state::SessionType;
                match session_type {
                    &SessionType::Administrator => Administrator {},
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

pub enum AdminOrApplicant {
    Admin,
    Applicant(i32),
}

impl AdminOrApplicant {
    pub fn can_access_applicant(&self, id: i32) -> bool {
        match self {
            &AdminOrApplicant::Admin => true,
            &AdminOrApplicant::Applicant(v) => v == id,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminOrApplicant {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        match Administrator::from_request(&request).await {
            Outcome::Success(_) => return Outcome::Success(AdminOrApplicant::Admin),
            _ => {}
        };

        match Applicant::from_request(&request).await {
            Outcome::Success(applicant) => {
                Outcome::Success(AdminOrApplicant::Applicant(applicant.applicant_id))
            }
            _ => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}

pub enum AdminOrProfessor {
    Admin,
    Professor(i32),
}

impl AdminOrProfessor {
    pub fn can_access_prof(&self, id: i32) -> bool {
        match self {
            &AdminOrProfessor::Admin => true,
            &AdminOrProfessor::Professor(v) => v == id,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminOrProfessor {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        match Administrator::from_request(&request).await {
            Outcome::Success(_) => return Outcome::Success(AdminOrProfessor::Admin),
            _ => {}
        };

        match Professor::from_request(&request).await {
            Outcome::Success(professor) => {
                Outcome::Success(AdminOrProfessor::Professor(professor.professor_id))
            }
            _ => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}

pub struct LoggedIn {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoggedIn {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        match Administrator::from_request(&request).await {
            Outcome::Success(_) => return Outcome::Success(LoggedIn {}),
            _ => {}
        };

        match Applicant::from_request(&request).await {
            Outcome::Success(_) => return Outcome::Success(LoggedIn {}),
            _ => {}
        };

        match Professor::from_request(&request).await {
            Outcome::Success(_) => Outcome::Success(LoggedIn {}),
            _ => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
