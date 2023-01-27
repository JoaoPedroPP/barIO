mod manipulation;

fn main() {
    dotenv::dotenv().ok();
    manipulation::convert("pikachu_p.jpg");
    // manipulation::convert("test.png");
}
