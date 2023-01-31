use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next, Result};
use task_local_extensions::Extensions;

struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let url = req.url().clone();

        println!("request {url}");

        let response = next.run(req, extensions).await;

        if let Ok(response) = &response {
            let header_x_cache = response.headers().get("x-cache").unwrap();

            println!("response {header_x_cache:?} {url}");
        }

        response
    }
}

pub(crate) fn create() -> ClientWithMiddleware {
    ClientBuilder::new(Client::new())
        .with(LoggingMiddleware)
        .with(Cache(HttpCache {
            // TODO don't leave it as ForceCache
            mode: CacheMode::ForceCache,
            // TODO where do I want the cache?
            manager: CACacheManager::default(),
            options: None,
        }))
        .build()
}
