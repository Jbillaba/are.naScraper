struct Image {
    image: Option<String>,
    text: Option<String>,
}

fn grab(){
    let response = reqwest::blocking::get("https://www.are.na/andreia-de-matos/food-illustration-dszkfpll53g");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    
    dbg!(document);

}

fn main() {
    grab();
}


