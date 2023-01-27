mod manipulation;

fn main() {
    dotenv::dotenv().ok();
    let img: String = manipulation::convert("pikachu_p.jpg");
    // manipulation::convert("test.png");
    match manipulation::save_redis(img) {
        Ok(_) => println!("Sucesso no redis"),
        Err(_) => println!("Sem sucesso no redis"),
    };
}
