use warp::Filter;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let hello = warp::path!("whatsmyip")
        .and(warp::filters::addr::remote())
        .and_then(|addr: Option<SocketAddr>| async move {
            match addr {
                Some(t) => Ok(format!("{}", t.ip())),
                None => Err(warp::reject::not_found())
            }
        });

    warp::serve(hello)
        .run(([0, 0, 0, 0], 1337))
        .await;
}
