use std::{env, fs, process};

fn phraser(codex: String) {
    println!("fn main() {{");

    for line in codex.lines() {
        let rust_line = converter(line);
        if rust_line.trim().is_empty() {
            println!();
        } else {
            println!("    {}", rust_line);
        }
    }

    println!("}}");
}

fn converter(line: &str) -> String {
    let mut out = line.to_string();

    if out.contains("function") {
        out = out
            .replace("function", "fn")
            .replace(",", ": i32,")
            .replace(") {", ": i32) -> i32 {");
    }

    if out.contains("const") {
        out = out.replace("const", "let").replace(" = ", ": i32 = ");
    }

    if out.contains("console.log(") {
        out = out.replace("console.log(", "println!(\"{}\", ");
    }

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String;

    if args.len() < 2 {
        filename = String::from("D:\\Users\\Lithium\\Desktop\\University\\Sistemas operativos\\Tarea\\RVM\\src\\codex.js");
    } else {
        filename = args[1].clone();
    }

    let js_code = match fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("No se pudo leer el archivo '{}': {}", filename, e);
            process::exit(1);
        }
    };

    phraser(js_code);
}