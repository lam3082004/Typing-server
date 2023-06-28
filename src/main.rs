use std::net::SocketAddr;
// use reqwest::Response;
use scraper::{Html, Selector};
use axum::Router;
use axum::response;
use axum::routing::get;
// use derive_new::new;
// use serde::{Serialize}; 
// mod data;

#[tokio::main]
async fn main() {
    let mut names = Vec::new();
        names.push("cinoss");
        names.push("long29031997");
        names.push("phiquan1606");
        names.push("lam3082004");
        names.push("cuongal");
        names.push("imakite");
        names.push("tuht97");
        names.push("mikamokedo");
        names.push("anhchile");
        names.push("lovehary90");
        names.push("tainh274");
        names.push("phieulong97");

    // let mut kq = Vec::new();
    // for username in names {
    //     kq.push(call_data_racer_api(username.to_string()).await);
    // }
    let mut kq1 = Vec::new();
    for i in 0..names.len(){
        let kq: Vec<String> = call_data_racer_api(names[i].to_string()).await;
        kq1.push(kq);
        // kq.join(" ");
    }

    let routes_hello = Router::new().route(
        "/hello",
get(|| async { response::Json(kq1) }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> listen on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}  

// #[derive(new, Debug)]
// struct Data {
//     pub name: String,
//     pub response: Response,
// }

 async fn call_data_racer_api(name: String) -> Vec<String>{
    let mut url = "https://typeracerdata.com/profile?username=".to_string();
    url.push_str(&name);

    let res = reqwest::get(url).await.unwrap();

    let mut data = Vec::new();
    let mut a = 1;
    let mut check = Vec::new();
    let doc_body = Html::parse_document(&res.text().await.unwrap());
    let kq = Selector::parse(".r").unwrap();
    data.push(name);

    for kq in doc_body.select(&kq){
        let kqs = kq.text().collect::<Vec<_>>();
        // print!("{:#?}", kqs[0]);
        check.push(kqs[0])
    }
    for i in check{
        if a == 0{
            // println!("{:#?}", i);    
            data.push(i.to_string());
        }
        if i == "Top marathon"{
            a = 0;
        }
    }
    data
}