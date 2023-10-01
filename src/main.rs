/*use image::{GenericImageView, DynamicImage};
use image::imageops::FilterType;*/

use std::thread;
use tiny_http::{Method, Request, Response, Server};
use std::collections::HashMap;

use std::fs::File;

use image;
use image::GenericImageView;

fn main() {
    /*
    let img = image::open("tests/pic.jpg").unwrap();

    let soft = resize(&img, Some(50), Some(50), true);
    let hard = resize(&img, Some(50), Some(50), false);

    println!("Original {:?}", img.dimensions());
    println!("Soft {:?}", soft.dimensions());
    println!("Hard {:?}", hard.dimensions());
    */

    let server = Server::http("0.0.0.0:8000").unwrap();

    println!("Server running...");

    loop {
        if let Ok(Some(request)) = server.try_recv() {
            thread::spawn(move || {
                handle_request(request);
            });
        }
    }
}

fn has_img_cont_type(request: &Request) -> bool {
    let content_type = request
        .headers()
        .iter()
        .find(|header| header.field.equiv("Content-Type"))
        .map(|header| header.value.to_owned());

    if content_type.is_none() {
        return false;
    }

    if !content_type.unwrap().to_string().starts_with("image/") {
        return false;
    }

    true
}

fn parse_query(raw_query: String) -> HashMap::<String, String> {
    let mut clean_query = raw_query.replace("/", "");
    clean_query = clean_query.replace("?", "");

    let raw_key_values = clean_query.split("&");

    let mut parsed_query = HashMap::<String, String>::new();
    for rkv in raw_key_values {
        let key_value: Vec::<_> = rkv.split("=").collect();

        if key_value.len() == 2 {
            parsed_query.insert(key_value[0].to_string(), key_value[1].to_string());
        }
    }

    parsed_query
}

fn parse_dimensions(raw_dimensions: String) -> std::result::Result<(u32, u32), ()> {
    let parsed_dimensions: Vec::<_> = raw_dimensions.split("x").collect();

    if parsed_dimensions.len() != 2 {
        return Err(());
    }

    match parsed_dimensions[0].parse::<u32>() {
        Ok(width) => {
            match parsed_dimensions[1].parse::<u32>() {
                Ok(height) => Ok((width, height)),
                Err(_) => {
                    return Err(());
                }
            }
        },
        Err(_) => {
            return Err(());
        }
    }
}

fn abort_request(request: Request, reason: &str) {
    let response = Response::from_string(reason);
    request.respond(response).unwrap();
}

fn handle_request(mut request: Request) {
    if *request.method() != Method::Post {
        abort_request(request, "Request Method Must be Post");
        return;
    }

    if !has_img_cont_type(&request) {
        abort_request(request, "Expected `image/*` Content-Type");
        return;
    }


    //
    let mut bytes: Vec::<u8> = vec![];
    request.as_reader().read_to_end(&mut bytes);
    let cursor = std::io::Cursor::new(bytes);

    if let Ok(img) = image::io::Reader::new(cursor).with_guessed_format().expect("").decode() {
        let (w, h) = img.dimensions();
        println!("Auto Dimensions: {} x {}", w, h);
    }
    //

    /* let mut file = File::create("tests/img2.png").unwrap();
    if let Err(e) = std::io::copy(&mut request.as_reader(), &mut file) {
        eprintln!("Error: Couldn't copy from stream to file => {:?}", e);
        abort_request(request, "Internal Server Error");
        return;
    }*/

    let mut parsed_query = parse_query(request.url().to_string());

    if let Some(dim) = parsed_query.get_mut("dim") {
        match parse_dimensions(dim.to_string()) {
            Ok((width, height)) => println!("{width}x{height}"),
            Err(_) => {
                abort_request(request, "Invalid Dimensions");
                return;
            }
        }
    } else {
        abort_request(request, "Expected Resize Dimensions `WxH`");
        return;
    }

    let response = Response::from_string("Hello PicZer");
    request.respond(response).unwrap();
}

/*
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
*/
