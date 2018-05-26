extern crate actix_web;
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{server, App, HttpRequest, http};
use std::cell::Cell;

struct AppState {
    counter: Cell<usize>
}

fn index(req: HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);

    format!("Request number: {}", count)
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        vec![
            App::with_state(AppState { counter: Cell::new(0) } )
                .prefix("/counter")
                .resource("/", |r| r.method(http::Method::GET).f(index))
                .boxed(),
            App::with_state(AppState { counter: Cell::new(0) } )
                .prefix("/counter2")
                .resource("/", |r| r.method(http::Method::GET).f(index))
                .boxed(),
        ]
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run();
}

