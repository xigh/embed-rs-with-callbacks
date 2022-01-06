use hyper::{Client, Uri, Method, Request, Body};
use hyper_tls::HttpsConnector;
use http::uri::Scheme;

pub struct GetResultImpl {
    pub status: i32,
    pub body: String,
}

pub struct GetErrorImpl {
    pub status: i32,
    pub body: String,
}

pub async fn http_get<S: Into<String>>(url: S, cb: impl Fn(Option<&GetResultImpl>, Option<&GetErrorImpl>)) {
    let url = url.into();
    let url = Uri::try_from(url);
    if let Err(err) = url {
        log::error!("http_get: could not parse url: {}", err);
        return;
    }
    let uri: Uri = url.unwrap();
    let scheme = uri.scheme();
    log::debug!("http_get: uri={}", uri);

    let req = Request::builder()
        .method(Method::GET)
        .uri(&uri)
        .header("Accept", "text/html")
        .body(Body::from(""))
        ;
    if let Err(err) = req {
        log::error!("http_get: could not create request: {}", err);
        return;
    }
    let req = req.unwrap();    

    log::debug!("http_get: waiting {:#?}", req);
    let response = if scheme == Some(&Scheme::HTTPS) {
        let https = HttpsConnector::new();
        Client::builder()
            .build::<_, hyper::Body>(https)
            .request(req)
    } else {
        Client::new()
            .request(req)
    };
    match response.await {
        Ok(res) => cb(Some(&GetResultImpl{
            status: res.status().as_u16() as i32,
            body: format!("{:#?}", res),
        }), None),
        Err(err) => cb(None, Some(&GetErrorImpl{
            status: 0,
            body: format!("{}", err),
        })),
    };
}
