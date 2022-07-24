use std::io::{stdout, Write};

mod animate;
mod lexer;
mod parser;

fn main() {
    // get source code file path from argument
    let args: Vec<String> = std::env::args().collect();
    let source_file_path = &args[1];
    // read source code file
    let source_code = std::fs::read_to_string(source_file_path).unwrap();

    let tokens = lexer::lex(&source_code);
    let (_, ast, _) = parser::parse(tokens).unwrap();
    let mut stdout = stdout();
    let r = animate::animate_eval(ast).unwrap();
    println!("\n\n\n");
    if !r.anim.is_empty() {
        print!(
            "{}                                                       ",
            r.anim[0]
        );
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    // animate with 0.1 sec delay
    for frame in r.anim {
        // replace previous frame
        print!(
            "\r{}                                                       ",
            frame
        );
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!(
        "\r{}{}{}",
        " ".repeat(r.last.0),
        r.last.1,
        " ".repeat(r.last.2)
    );
}
