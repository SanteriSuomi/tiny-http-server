use std::sync::Arc;

use super::{request::Request, response::Response};

#[derive(Clone)]
pub struct Route {
    pub func: Arc<dyn Fn(&Request, &mut Response) -> () + Send + Sync + 'static>,
}
