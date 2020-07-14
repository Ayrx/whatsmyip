use clap::{App, Arg};
use warp::Filter;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(Arg::with_name("PORT")
             .short("p")
             .long("port")
             .help("Listen on specified port.")
             .takes_value(true))
    .get_matches();

    let port = match matches.value_of("PORT") {
        Some(p) => p.parse().expect("unable to parse PORT"),
        None => 1337,
    };

    let hello = warp::path!("whatsmyip")
        .and(warp::filters::addr::remote())
        .and_then(|addr: Option<SocketAddr>| async move {
            match addr {
                Some(t) => Ok(format!("{}", t.ip())),
                None => Err(warp::reject::not_found())
            }
        });

    warp::serve(hello)
        .run(([0, 0, 0, 0], port))
        .await;
}
