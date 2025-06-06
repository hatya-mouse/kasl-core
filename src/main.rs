use knodiq_audio_shader::{Lexer, Parser};

fn main() {
    let input = "
    input buffer in_buffer = 0.0
    input float gain = 1.0
    output buffer out_buffer
    out_buffer = in_buffer * gain
    ";
    let lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let program = parser.parse();
    match program {
        Ok(tree) => {
            println!("Parsed successfully: {:?}", tree);
        }
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
        }
    }
}
