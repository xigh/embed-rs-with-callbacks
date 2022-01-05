use hyper::{Client, Uri, Method, Request, Body};
use std::ffi::CString;

pub async fn http_get<S: Into<String>>(url: S, cb: impl Fn(&CString)) {
    let url = url.into();
    let url = Uri::try_from(url);
    if let Err(err) = url {
        eprintln!("http_get: could not parse url: {}", err);
        return;
    }
    let uri: Uri = url.unwrap();
    println!("http_get: uri={}", uri);

    let client = Client::new();
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "text/html")
        .body(Body::from(""))
        ;
    if let Err(err) = req {
        eprintln!("http_get: could not create request: {}", err);
        return;
    }
    let req = req.unwrap();    

    println!("http_get: waiting {:#?}", req);
    let s = match client.request(req).await {
        Ok(res) => format!("http_get: Response: {:?}", res),
        Err(err) => format!("http_get: Error: {}", err),
    };
    let s = s.as_str();
    let cs = CString::new(s).unwrap(); // todo
    cb(&cs);
}
