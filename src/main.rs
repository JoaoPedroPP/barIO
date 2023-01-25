use std::fs;
use image::{GenericImageView, DynamicImage, Luma};

const SYMBOLS: [&str; 8] = ["*", "&", "#", "@", "$", "%", "?", "."];
// const SYMBOLS: [&str; 9] = ["?", "%", "$", "@", "#", "&", "*", "m", "k"];

#[derive(Debug, Clone)]
struct Hist {
    id: u8,
    value: u32,
}

impl Hist {
    fn default() -> Hist {
        Hist { id: 0, value: 0 }
    }
}

fn compute_histogram(img: &DynamicImage) -> Vec<Hist> {
    let mut histogram: Vec<Hist> = vec![Hist::default(); 256];
    histogram = histogram.into_iter().enumerate().map(|(x, _v)| Hist { id: x as u8, value: 0 }).collect();
    let binding = img.clone().into_luma8();

    binding.pixels()
        .map(|pixel| {
            let Luma([v]) = pixel;
            *v as usize
        })
        .for_each(|x| histogram[x].value += 1);
    histogram.sort_by(|a, b| b.value.cmp(&a.value));
    histogram
}

// fn normalize()

fn main() {
    let img = image::open("./pikachu_p.jpg").unwrap();
    // let img = image::open("./test.png").unwrap();
    let dims = img.dimensions();
    println!("Dims: {:?}", dims);
    let gray  = img.clone().into_luma8();
    let hist = compute_histogram(&img);
    let mut main_values: Vec<Hist> = hist[..SYMBOLS.len()].to_vec();
    main_values.reverse();
    println!("{:?}", main_values);
    let mut bars: Vec<Vec<String>> = Vec::new();
    let mut txt: Vec<String> = Vec::new();
    for i in 0..dims.1 {
        let mut row: Vec<String> = Vec::new();
        for j in 0..dims.0 {
            let Luma([v]) = gray.get_pixel(j,i);
            // println!("{:?}", *v);
            match main_values.binary_search_by(|pix| pix.id.cmp(v)) {
                Ok(pos) => row.push(SYMBOLS[(SYMBOLS.len()-1-pos) as usize].to_string()),
                Err(i_pos) => {
                    // println!("{:?}", i_pos);
                    match i_pos {
                        1 ..= 8 => {
                            let before = *v - main_values[i_pos-1].id as u8;
                            let after = main_values[i_pos].id as u8 - *v;

                            if after <= before {
                                row.push(SYMBOLS[(SYMBOLS.len()-i_pos) as usize].to_string());
                            } else {
                                row.push(SYMBOLS[(SYMBOLS.len()-1-i_pos) as usize].to_string());
                            }
                        },
                        _ => row.push(SYMBOLS[(SYMBOLS.len()-1-i_pos) as usize].to_string()),
                    };
                },
            }
        }
        let x: String = row.join(" ");
        txt.push(x);
    }
    // txt.reverse();
    let end: String = txt.join("\n").to_string();
    fs::write("foo.txt", end);
}
