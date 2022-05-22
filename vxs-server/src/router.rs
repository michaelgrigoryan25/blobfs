use std::convert::Infallible;

use hyper::Body;
use routerify::{Router, RouterService};

// use crate::metrics;

/// Returns a new central router which will define all request paths
/// with their corresponding handlers in one place.
fn routes() -> Router<Body, Infallible> {
    Router::builder()
        // .get("/metrics", metrics::handler)
        .build()
        .unwrap()
}

/// Converts all the routes from the central router into a service,
/// consumable by [hyper::Server].
pub(crate) fn service() -> anyhow::Result<RouterService<Body, Infallible>> {
    let service = RouterService::new(routes())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(service)
}
