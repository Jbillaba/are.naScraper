#[derive(Debug)]
struct Block {
    url: String,
    image: String,
}

fn main() {

    let mut blocks : Vec<Block> = Vec::new();

    let browser = headless_chrome::Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://www.are.na/andreia-de-matos/food-illustration-dszkfpll53g").unwrap();
    let html_blocks = tab.wait_for_elements("div.virtuoso-grid-item").unwrap();

    std::fs::write("screenshot.png", &screenshot_data).unwrap();

    
    for html_block in html_blocks {
        let url = html_block
            .wait_for_element("a")
            .unwrap()
            .get_attributes()
            .unwrap()
            .unwrap()
            .get(1)
            .unwrap()
            .to_owned();

        let image = html_block
            .wait_for_element("img")
            .unwrap()
            .get_attributes()
            .unwrap()
            .unwrap()
            .get(5)
            .unwrap()
            .to_owned();
        
        // let caption = html_block
        //     .wait_for_element("div.c-lgwVNZ c-dHTEdC c-lgwVNZ-iepcqn-ellipsis-true c-lgwVNZ-gyaQWK-size-xs c-lgwVNZ-ijkKOHM-css")
        //     .unwrap()
        //     .get_inner_text()
        //     .unwrap();

        let block = Block {
            url,
            image,
        };
        
        blocks.push(block);
        let count = blocks.len();
        println!("{count}")
    };

    let path = std::path::Path::new("blocks.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer
        .write_record(&["url","image"])
        .unwrap();

    for block in blocks {
        let url = block.url;
        let image = block.image;

        writer.write_record(&[url, image]).unwrap();
    }

    writer.flush().unwrap();
    

}

