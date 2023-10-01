use image::{GenericImageView, DynamicImage};
use image::imageops::FilterType;

fn main() {
    let img = image::open("tests/pic.jpg").unwrap();

    let soft = resize(&img, Some(50), Some(50), true);
    let hard = resize(&img, Some(50), Some(50), false);

    println!("Original {:?}", img.dimensions());
    println!("Soft {:?}", soft.dimensions());
    println!("Hard {:?}", hard.dimensions());
}

fn resize(img: &DynamicImage, w: Option<u32>, h: Option<u32>, aspct_ratio: bool) -> DynamicImage {
    let (original_width, original_height) = img.dimensions();
    let new_width = w.unwrap_or(original_width);
    let new_height = h.unwrap_or(original_height);
    if aspct_ratio {
        soft_resize(img, new_width, new_height)
    } else {
        hard_resize(img, new_width, new_height)
    }
}

fn soft_resize(img: &DynamicImage, w: u32, h: u32) -> DynamicImage {
    img.resize(w, h, FilterType::Lanczos3)
}

fn hard_resize(img: &DynamicImage, w: u32, h: u32) -> DynamicImage {
    img.resize_exact(w, h, FilterType::Lanczos3)
}
