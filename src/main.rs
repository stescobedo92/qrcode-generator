use qrcode::QrCode;
use image::Luma;
use std::env;

fn parse_arguments(args: Vec<String>) {
    //Verify that there are at least two arguments (the command and the text to convert).
    if args.len() < 3 {
        println!("Usage: {} <text to convert> <output path>", args[0]);
        return;
    }

    let text_to_convert = &args[1];
    let output_path = &args[2];

    generate_qr_code(text_to_convert, output_path);
}

fn generate_qr_code(text: &str, output_path: &str) {
    let code = QrCode::new(text.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save(output_path).unwrap();
}

fn main(){
    let args: Vec<String> = env::args().collect();
    parse_arguments(args);
}