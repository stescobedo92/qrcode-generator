use qrcode::QrCode;
use image::Luma;
use std::env;

/// Parses the command-line arguments provided as a vector of strings.
///
/// # Arguments
///
/// * `args` - A vector of strings representing the command-line arguments. It is expected
///            to contain at least three elements: the program name, the text to convert,
///            and the output path.
///
/// # Panics
///
/// Panics if the number of arguments is not exactly three, with a message displaying the
/// expected usage.
///
/// Panics if the text to convert is an empty string, indicating that it cannot be processed.
///
/// # Examples
///
/// ```rust
/// let args = vec![
///     String::from("program_name"),
///     String::from("Hello, world!"),
///     String::from("output.png")
/// ];
/// parse_arguments(args);
/// ```
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

/// Generates a QR code image from the given text and saves it to the specified output path.
///
/// # Arguments
///
/// * `text` - A string slice containing the text to be converted into a QR code.
///
/// * `output_path` - A string slice representing the path where the generated QR code image
///                   will be saved.
///
/// # Panics
///
/// Panics if there is an error while generating the QR code or saving the image to the output path.
///
/// # Examples
///
/// ```rust
/// let text = "Hello, world!";
/// let output_path = "qrcode.png";
/// generate_qr_code(text, output_path);
/// ```
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