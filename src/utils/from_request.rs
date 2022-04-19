use rocket::request::{self, Request, FromRequest};
use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct RequestId {
    pub number: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r RequestId {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(request.local_cache(|| {
            RequestId {
                number: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            }
        }))
    }
}
