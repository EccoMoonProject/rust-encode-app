extern crate base64;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use base64::{decode, encode};
use reqwest;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::stdin;
use std::str;
use std::time::Duration;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let client = reqwest::Client::new();
//     let forms = client
//         .get("http://localhost:7500/form")
//         .header("Accept", "text/plain")
//         .timeout(Duration::from_secs(3))
//         .send()
//         .await?
//         .text()
//         .await?;

//     fs::write("src/inp.json", encode(forms)).expect("Unable to write file");
//     // Declare a mutable input string

//     Ok(())
// }

// #[get("/")]
// async fn hello() -> impl Responder {
//     let data = fs::read_to_string("src/inp.json").expect("Unable to read file");
//     println!("Your encoded data is: {}", data);
//     HttpResponse::Ok().body(data)
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
fn main() {
    //Declare a mutable input string
    println!("Name,Certificate,Date:");
    let mut name_string = String::new();
    stdin()
        .read_line(&mut name_string)
        .ok()
        .expect("Failed to read line");

    // println!("{}", encode(input_string));

    let data = encode(name_string).to_string();
    println!("Your encoded data is: {}", data);
    fs::write("src/inp.json", data).expect("Unable to write file");

    // Declare a mutable input string
    println!("Press any key to decode your data");
    let mut type_user = String::new();
    stdin()
        .read_line(&mut type_user)
        .ok()
        .expect("Failed to read line");

    if type_user != "" {
        let data = fs::read_to_string("src/inp.json").expect("Unable to read file");

        let decoded = match base64::decode(data) {
            Ok(v) => String::from_utf8(v).ok(),
            _ => None,
        };
        println!("{:?}", decoded);
    }
}
