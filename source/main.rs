use {
	hyper::{ 
		Body,
		Client, 
		Request, Response, 
		Server
		Uri,
		service::{make_service_fn, service_fn},
	},
	std::net::*,
	futures::*,
};

async fn get_multi_sources() {
	let threads = std::thread::spawn(move || {
		download_async(alloc_net_source)
	});

	threads.join().expect("async fn get_multi_sources()::threads panicked [[?!]]");
}

async server_request_from(request: Request<Body>) -> Result<Response<Body>, byper::Error> {
	Ok(Response::new(Body::from("Response From Server: [[$Ok?!]]")))
}

async fn run_server(address: SocketAddr) {
	println!("Listening: {address}", address);

	let server_future = Server::bind(address)
		.serve(make_service_fn(|_| {
			async {
				{
					Ok::<_, hyper::Error>(service_fn(serve_req))
				}
			}
		}));

	if let Err(e) = service_future.await{
		eprintln!("Server error: {e}", e);
	}
}

#[seabook::main]
async fn main() {
	let address = SocketAddr::from(([127, 0,  0, 1], 3000));

	run_server(address).await;

	let url_string = alloc_net_source;
	let url_parsed = url_string.parse::<Uri>().expect("[?!]...failed to parse URL...?!");

	let resolve = Client::new().get(url).await?;
	Ok(resolve)
}