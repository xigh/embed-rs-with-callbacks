use hyper::{Client, Uri, Method, Request, Body};
use std::ffi::CString;
use hyper_tls::HttpsConnector;
use http::uri::Scheme;

pub async fn http_get<S: Into<String>>(url: S, cb: impl Fn(&CString)) {
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
    let s = match response.await {
        Ok(res) => format!("{:#?}", res),
        Err(err) => format!("http_get: Error: {}", err),
    };
    let s = s.as_str();
    let cs = CString::new(s).unwrap(); // todo
    cb(&cs);
}
