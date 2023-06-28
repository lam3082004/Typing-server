use scraper::{Html, Selector};
fn main() {
    let mut names: Vec<&str> = Vec::new();
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

    for username in names {
        println!("{:#?}",call_data_racer_api(username.to_string()));
    }
}  
fn call_data_racer_api(name: String) -> Vec<String>{
     let mut url = "https://typeracerdata.com/profile?username=".to_string();
    url.push_str(&name);
    let res = reqwest::blocking::get(
        url,
    )
    .unwrap();

    let mut a = 1;
    let mut data= Vec::new();
    let mut check = Vec::new();
    let doc_body = Html::parse_document(&res.text().unwrap());
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