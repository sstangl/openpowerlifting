use std::ops::Deref;

use rocket::http::hyper::header::REFERER;
use rocket::http::uri::Absolute;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

/// Extracts the path specified in the `referer` header.
#[derive(Clone, Debug)]
pub struct ReferringPath {
    inner: String,
}

impl Deref for ReferringPath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[rocket::async_trait]
impl<'request> FromRequest<'request> for ReferringPath {
    type Error = ();

    async fn from_request(req: &'request Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the `referer` header and parse to a URI
        let Some(Ok(referrer)) = req.headers().get_one(REFERER.as_str()).map(Absolute::parse)
        else {
            return Outcome::Failure((Status::Ok, ()));
        };

        // Get the path component
        let inner = referrer.path().as_str().to_owned();

        Outcome::Success(ReferringPath { inner })
    }
}
