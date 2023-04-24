use std::sync::Arc;

use super::request::Request;

#[derive(Clone)]
pub struct Middleware {
    pub func: Arc<dyn Fn(Option<&Middleware>, &mut Request) -> () + Send + Sync + 'static>,
}

impl Middleware {
    pub fn new(
        func: impl Fn(Option<&Middleware>, &mut Request) -> () + Send + Sync + 'static + 'static,
    ) -> Self {
        Self {
            func: Arc::new(func),
        }
    }
}
