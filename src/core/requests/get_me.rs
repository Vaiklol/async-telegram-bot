use crate::core::other::User;
use super::Request;

use reqwest::r#async::multipart::Form;


#[derive(Debug, Constructor, PartialEq, Eq)]
pub struct GetMe {
    token: String,
}

impl Request<User> for GetMe {
    fn name(&self) -> &str { "getMe" }
    fn params(self) -> Option<Form> { None }
    fn token(&self) -> &str { &self.token }
}