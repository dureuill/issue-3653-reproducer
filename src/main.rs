use std::time::Duration;

use actix_web::{get, rt, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/sleepy-hello")]
async fn sleepy_hello() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5));
    HttpResponse::Ok().body("Hello... zZz... world")
}

fn main() {
    // replace the route with "hello" for the example to succeed
    const ROUTE: &str = "sleepy-hello";

    std::thread::Builder::new()
        .name("actix-rt".into())
        .spawn(|| {
            rt::System::new().block_on(
                HttpServer::new(|| App::new().service(hello).service(sleepy_hello))
                    .bind(("127.0.0.1", 8080))?
                    .run(),
            )
        })
        .unwrap();

    print!("Waiting for server to be up ...");
    loop {
        if ureq::get("http://127.0.0.1:8080/hello").call().is_ok() {
            break;
        }
        print!(".");
        std::thread::sleep(Duration::from_millis(100));
    }
    println!(" Server up!");

    let threads: Vec<_> = (0..24)
        .map(|i| {
            std::thread::spawn(move || {
                let res = ureq::get(&format!("http://127.0.0.1:8080/{ROUTE}")).call();
                match res {
                    Ok(res) => {
                        println!(
                            "[Client thread {i}]Called '{ROUTE}', got {}",
                            res.into_string().unwrap()
                        );
                        1
                    }
                    Err(err) => {
                        println!("[Client thread {i}]Called '{ROUTE}', got {err}");
                        0
                    }
                }
            })
        })
        .collect();

    let count = threads.len();
    let succeeded: usize = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .sum();
    println!("Succeeded {succeeded}/{count}");
    assert_eq!(succeeded, count);
}
