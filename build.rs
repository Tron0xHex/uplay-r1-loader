use regex::*;
use std::{env, fs, path::Path};
use voca_rs::case;

fn main() {
    let mut hooks = vec!["&[\n".into()];

    for entry in fs::read_dir("src/exports").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let is_file = path.is_file();
        let extension = path.extension().unwrap();

        if is_file && extension == "rs" {
            let content = fs::read_to_string(&path).unwrap();

            for line in content.split("\n") {
                if line.contains("export_name") {
                    let reg_expr = Regex::new(r#"(?m)["](.*)["]"#).unwrap();
                    let reg_match = reg_expr.find(&line).unwrap();

                    let name = &line[reg_match.start()..reg_match.end()];
                    let rust_name = case::snake_case(&name);

                    hooks.push(format!(
                        "\t({}, UplayPtr({} as * const())),\n",
                        &name, &rust_name
                    ));
                }
            }
        }
    }

    hooks.push("]".into());

    let out_dir = env::var("OUT_DIR").unwrap();
    let hooks_path = Path::new(&out_dir).join("hooks.rs");
    let hooks_content = hooks.join("");

    fs::write(&hooks_path, &hooks_content).unwrap();
}
