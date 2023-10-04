# PicZer :: (Image Resizer Rust App)

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Configuration](#configuration)
- [Dependencies](#dependencies)
- [Contributing](#contributing)
- [License](#license)

---

## Introduction

Piczer is a powerful and efficient Rust-based HTTP service that allows you to resize images on-the-fly. Whether you need to quickly generate thumbnails or adapt images for your web application, this server simplifies the process.

## Features

- **Image Resizing**: Resize images with ease.
- **Customization**: Specify dimensions and choose whether to preserve the aspect ratio.
- **Multiple Formats**: Support for various image formats.
- **Easy Integration**: Seamlessly incorporate image resizing into your projects.

## Getting Started

To start using the Image Resizing Server, follow these simple steps:

1. **Clone the Repository**:
```shell
git clone https://github.com/edilson258/piczer.git
cd piczer
```

2. **Build the Project**:
```shell
cargo build --release
```

3. **Run the Server**:
```shell
cargo run --release
```

The server will launch and listen on `http://0.0.0.0:8000`.

## Usage

You can utilize the Image Resizing Server to resize images via HTTP POST requests. Refer to the [API Endpoints](#api-endpoints) section for detailed instructions on making requests.

## API Endpoints

- **POST /?dim=WxH&ar=true**: Resize an image.
  - `dim` (required): Specify the new dimensions in the format `WxH` (e.g., `300x200`).
  - `ar` (optional): Set to `true` to preserve the aspect ratio (default is `true`).

Example request using `curl`
```shell
curl -X POST -H "Content-Type: application/octet-stream" --data-binary @<path/to/image>
"http://127.0.0.1:8000/?dim=500x500" --output "resized.png" -v
```
Note: replace `<path/to/image>` with the path of your image.

Example request using `node script`:
```shell
node examples/node-client/main.js
```

## Configuration

Customize the server settings by modifying `src/main.rs`. You can adjust parameters such as the listening address and port to suit your needs.

## Dependencies

This project relies on the following Rust crates:

- `tiny_http`: A lightweight HTTP server library.
- `image`: A versatile crate for image processing.

## Contributing

We welcome contributions! If you have suggestions for improvements or bug fixes, please open an issue or submit a pull request. Be sure to adhere to our code of conduct.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
