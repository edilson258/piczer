use image;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use std::collections::HashMap;
use std::thread;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

fn main() {
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

    if content_type.unwrap().to_string() != String::from("application/octet-stream") {
        return false;
    }

    true
}

fn parse_query(raw_query: String) -> HashMap<String, String> {
    let mut clean_query = raw_query.replace("/", "");
    clean_query = clean_query.replace("?", "");

    let raw_key_values = clean_query.split("&");

    let mut parsed_query = HashMap::<String, String>::new();
    for rkv in raw_key_values {
        let key_value: Vec<_> = rkv.split("=").collect();

        if key_value.len() == 2 {
            parsed_query.insert(key_value[0].to_string(), key_value[1].to_string());
        }
    }

    parsed_query
}

fn parse_dimensions(raw_dimensions: String) -> std::result::Result<(u32, u32), ()> {
    let parsed_dimensions: Vec<_> = raw_dimensions.split("x").collect();

    if parsed_dimensions.len() != 2 {
        return Err(());
    }

    match parsed_dimensions[0].parse::<u32>() {
        Ok(width) => match parsed_dimensions[1].parse::<u32>() {
            Ok(height) => Ok((width, height)),
            Err(_) => {
                return Err(());
            }
        },
        Err(_) => {
            return Err(());
        }
    }
}

fn parse_can_keep_aspect_ration(can_keep_aspcet_ration: &str) -> std::result::Result<bool, ()> {
    match can_keep_aspcet_ration {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(()),
    }
}

fn abort_request(request: Request, reason: &str) {
    let header = Header::from_bytes(&b"Content-Type"[..], &b"text/palin"[..]).unwrap();
    let mut headers: Vec<Header> = Vec::new();
    headers.push(header);
    let reason_bytes = reason.as_bytes();

    let response = Response::new(StatusCode(400), headers, reason_bytes, None, None);

    request.respond(response).unwrap();
}

fn extract_image_from_request(request: &mut Request) -> std::result::Result<DynamicImage, ()> {
    let mut bytes: Vec<u8> = vec![];

    if let Err(e) = request.as_reader().read_to_end(&mut bytes) {
        eprintln!("Error: Couldn't to read image data to buffer: {:?}", e);
        return Err(());
    }

    let cursor = std::io::Cursor::new(bytes);
    let raw_image = image::io::Reader::new(cursor);

    let guessed_format = match raw_image.with_guessed_format() {
        Ok(image) => image,
        Err(_) => return Err(()),
    };

    match guessed_format.decode() {
        Ok(image) => Ok(image),
        Err(_) => Err(()),
    }
}

fn handle_request(mut request: Request) {
    if *request.method() != Method::Post {
        abort_request(request, "Request Method Must be Post");
        return;
    }

    if !has_img_cont_type(&request) {
        abort_request(request, "Expected `application/octet-stream` Content-Type");
        return;
    }

    let mut parsed_query = parse_query(request.url().to_string());

    let recv_dimensions = match parsed_query.get_mut("dim") {
        Some(dimensions) => match parse_dimensions(dimensions.to_string()) {
            Ok((width, height)) => (width, height),
            Err(_) => {
                abort_request(request, "Invalid Dimensions");
                return;
            }
        },
        None => {
            abort_request(request, "Expected Resize Dimensions `WxH`");
            return;
        }
    };

    let can_keep_aspcet_ration = match parsed_query.get_mut("ar") {
        Some(can_keep_aspcet_ration) => {
            match parse_can_keep_aspect_ration(can_keep_aspcet_ration.as_str()) {
                Ok(can_keep_aspcet_ration) => can_keep_aspcet_ration,
                Err(_) => {
                    eprintln!("Error: Couldn't parse can keep aspect ratio");
                    abort_request(
                        request,
                        "Invalid Aspect Ration, it can be `true` or `false`",
                    );
                    return;
                }
            }
        }
        None => true,
    };

    let recv_image = match extract_image_from_request(&mut request) {
        Ok(image) => image,
        Err(_) => {
            eprintln!("Error: Couldn't Extact Image from request");
            abort_request(
                request,
                "Unknown Error Occured, make sure that the image is valid",
            );
            return;
        }
    };

    let (new_width, new_height) = recv_dimensions;
    let resized_image = resize(
        &recv_image,
        Some(new_width),
        Some(new_height),
        can_keep_aspcet_ration,
    );

    // logs
    println!();
    println!("======================");
    println!("[+] Original Dimensions {:?}", recv_image.dimensions());
    println!("[+] New Dimensions {:?}", recv_dimensions);
    println!("[+] Aspect Ration: {can_keep_aspcet_ration}");
    println!(
        "[+] Dimensions After Resize: {:?}",
        resized_image.dimensions()
    );

    let output_path = "tests/output.png";

    match resized_image.save(&output_path) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: Couldn't save resized image to fs: {:?}", e);
            abort_request(request, "Intenal Server Error");
            return;
        }
    }

    match std::fs::File::open(&output_path) {
        Ok(file) => {
            request.respond(Response::from_file(file)).unwrap();
        }
        Err(e) => {
            eprintln!("Error: Couldn't open resized image: {:?}", e);
            abort_request(request, "Intenal Server Error");
            return;
        }
    }
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
