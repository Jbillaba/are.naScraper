#[derive(Debug)]
struct Block {
    image: Option<String>,
}

fn grab(){
    let response = reqwest::blocking::get("https://www.are.na/andreia-de-matos/food-illustration-dszkfpll53g");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);

    let html_block_selector = scraper::Selector::parse("div.PJLV").unwrap();
    let html_blocks = document.select(&html_block_selector);
    let mut blocks: Vec<Block> = Vec::new();

    for html_block in html_blocks {
        let image = html_block
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);
        

        let block = Block {
            image,
        };
        
        blocks.push(block);
    };

    println!("{:?}", blocks);

    let path = std::path::Path::new("blocks.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer
        .write_record(&["image"])
        .unwrap();

    for block in blocks {
        let image = block.image.unwrap();
        writer.write_record(&[image]).unwrap();
    }

    writer.flush().unwrap();
    

}

fn main() {
    grab();
}


