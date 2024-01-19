use hyper::{
    body::Body, header::CONTENT_TYPE, server::conn::Http, service::service_fn, Method, Request,
    Response, StatusCode,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

fn create_default_response(value: String) -> anyhow::Result<Response<Body>> {
    let (mut parts, _) = Response::<Body>::default().into_parts();

    parts.headers.insert(CONTENT_TYPE, "json".parse().unwrap());

    let body = Body::from(value);

    Ok(Response::from_parts(parts, body))
}

async fn handler(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let value = String::from("{ \"field\": \"value\"}");
            create_default_response(value)
        }
        (&Method::GET, "/get") => {
            let query = req.uri().query().unwrap();

            let parts: Vec<&str> = query.split('&').collect();

            let list: Vec<_> = parts
                .into_iter()
                .map(|key_value| {
                    let arr: Vec<_> = key_value.split('=').collect();
                    if arr.len() != 2 {
                        return Err(anyhow::anyhow!("Unexpected key-value pair, {}", key_value));
                    }

                    let (key, value) = (arr[0], arr[1]);

                    Ok(format!("\"{}\": \"{}\"", key, value))
                })
                .collect::<anyhow::Result<Vec<String>>>()?;

            let value = format!("{{ {} }}", list.join(", "));
            create_default_response(value)
        }
        (&Method::POST, "/post") => {
            let body = hyper::body::to_bytes(req.body_mut()).await?;

            println!("Body is {:#?}", body);

            let value: serde_json::Value = serde_json::from_slice(&body)?;

            println!("Parsed value is {:?}", value);
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap())
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
    }
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Public api initialised");

    let addr = SocketAddr::from(([127, 0, 0, 1], 9007));

    let listener = TcpListener::bind(addr).await?;
    println!("listegning on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(error) = Http::new()
                .serve_connection(stream, service_fn(handler))
                .await
            {
                println!("An error has occured: {}", error);
            }
        });
    }
}
