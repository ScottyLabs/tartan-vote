use http::{HeaderValue, Method, header};
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin: &HeaderValue, _parts| {
            origin_allowed(origin)
        }))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT, header::COOKIE])
        .allow_credentials(true)
}

fn origin_allowed(origin: &HeaderValue) -> bool {
    let Ok(origin) = origin.to_str() else {
        return false;
    };

    let Some((scheme, rest)) = origin.split_once("://") else {
        return false;
    };

    if scheme != "http" && scheme != "https" {
        return false;
    }

    let host = rest.split('/').next().unwrap_or(rest);
    let host = host.split(':').next().unwrap_or(host);

    host == "localhost"
        || host == "127.0.0.1"
        || host == "tartan.vote"
        || host.ends_with(".tartan.vote")
        || host.ends_with(".scottylabs.org")
        || host.ends_with(".scottylabs.net")
        || host == "scottylabs.org"
        || host == "scottylabs.net"
}
