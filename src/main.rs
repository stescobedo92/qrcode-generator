use qrcode::QrCode;
use image::Luma;
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    //Verify that there are at least two arguments (the command and the text to convert).
    if args.len() < 3 {
        println!("Usage: {} <text to convert> <output path>", args[0]);
        return;
    }

    let text_to_convert = args[1].as_bytes();
    let output_path = &args[2];
    let code = QrCode::new(text_to_convert).unwrap();

    let image = code.render::<Luma<u8>>().build();
    image.save(output_path).unwrap();
}