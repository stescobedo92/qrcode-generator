use qrcode::QrCode;
use image::Luma;
use std::env;

fn parse_arguments(args: Vec<String>) {
    //Verify that there are at least two arguments (the command and the text to convert).
    if args.len() != 3 {
        panic!("Usage: {} <text to convert> <output path>", args[0]);
    }

    let text_to_convert = &args[1];
    let output_path = &args[2];

    if text_to_convert.is_empty() {
        panic!("Error: Text to convert cannot be empty");
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_qr_generation() {
        let text = "Test text";
        let output_path = "test_qr.png";
        generate_qr_code(text, output_path);
        assert!(fs::metadata(output_path).is_ok());
        fs::remove_file(output_path).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_invalid_arguments() {
        let args = vec!["program_name".to_string()];
        parse_arguments(args);
    }
    #[test]
    #[should_panic]
    fn test_invalid_text() {
        let args = vec!["program_name".to_string(), "".to_string(), "output.png".to_string()];
        parse_arguments(args);
    }
    #[test]
    #[should_panic]
    fn test_invalid_output_path() {
        let args = vec!["program_name".to_string(), "text".to_string()];
        parse_arguments(args);
    }
}