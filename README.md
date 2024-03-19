# qrcode-generator
This is a simple Rust program that generates QR codes from user-provided text. The program utilizes the `qrcode` library to generate the QR code and the `image` library to render it and save it as a grayscale image.

# Usage
The program is executed from the command line and accepts two arguments:

```rust
cargo run <text to convert> <output path>
```
- `<text to convert>`: The text to convert into a QR code.
- `<output path>`: The path where the generated QR code image will be saved.

For example

```rust
cargo run "Hello, world!" qrcode.png
```
This will generate a QR code for the text "Hello, world!" and save it as an image named qrcode.png in the current directory.

# Dependencies
The project depends on the following libraries:

`qrcode`: To generate the QR codes.
`image`: To render and save the QR code images.

# Contributions
Contributions are welcome. If you have ideas to improve this program or encounter any issues, feel free to open an issue or submit a pull request.