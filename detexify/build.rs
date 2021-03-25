extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use yaml_rust::YamlLoader;

#[derive(Debug)]
struct Symbol {
    command: String,
    package: String,
    font_encoding: String,
    text_mode: bool,
    math_mode: bool,
}

impl Symbol {
    fn get_id(&self) -> &'static str {
        let id = format!(
            "{}-{}-{}",
            self.package,
            self.font_encoding,
            self.command.replace("\\", "_")
        );

        // TODO: remove this once https://github.com/sfackler/rust-phf/pull/185 is merged
        Box::leak(
            base32::encode(base32::Alphabet::RFC4648 { padding: false }, id.as_bytes())
                .into_boxed_str(),
        )
    }
}

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("symbol_table.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut docs = YamlLoader::load_from_str(include_str!("symbols.yaml")).unwrap();
    let doc = docs.pop().unwrap();

    let mut builder = phf_codegen::Map::new();

    for symbol in doc.into_vec().unwrap() {
        if let Some(symbol) = symbol.clone().into_string() {
            let s = Symbol {
                command: symbol,
                package: "latex2e".to_string(),
                font_encoding: "OT1".to_string(),
                text_mode: true,
                math_mode: false,
            };

            builder.entry(s.get_id(), &format!("{:?}", s));
        } else {
            let mut package = "latex2e";
            if !symbol["package"].is_badvalue() {
                package = symbol["package"].as_str().unwrap();
            }

            let mut font_enc = "OT1";
            if !symbol["fontenc"].is_badvalue() {
                font_enc = symbol["fontenc"].as_str().unwrap();
            }

            if !symbol["bothmodes"].is_badvalue() {
                for symbol in symbol["bothmodes"].as_vec().unwrap() {
                    let s = Symbol {
                        command: symbol.clone().into_string().unwrap(),
                        package: package.to_string(),
                        font_encoding: font_enc.to_string(),
                        text_mode: true,
                        math_mode: true,
                    };
                    // panic!("{}", s.get_id());
                    builder.entry(s.get_id(), &format!("{:?}", s));
                }
            }

            if !symbol["textmode"].is_badvalue() {
                for symbol in symbol["textmode"].as_vec().unwrap() {
                    let s = Symbol {
                        command: symbol.clone().into_string().unwrap(),
                        package: package.to_string(),
                        font_encoding: font_enc.to_string(),
                        text_mode: true,
                        math_mode: false,
                    };

                    builder.entry(s.get_id(), &format!("{:?}", s));
                }
            }

            if !symbol["mathmode"].is_badvalue() {
                for symbol in symbol["mathmode"].as_vec().unwrap() {
                    let s = Symbol {
                        command: symbol.clone().into_string().unwrap(),
                        package: package.to_string(),
                        font_encoding: font_enc.to_string(),
                        text_mode: false,
                        math_mode: true,
                    };

                    if s.command == "\\lightning" && s.package == "wasysym" {
                        panic!(s.get_id());
                    }

                    // panic!("{}", s.get_id());
                    builder.entry(s.get_id(), &format!("{:?}", s));
                }
            }

            continue;
        }
    }

    write!(
        &mut file,
        "static SYMBOL_TABLE: phf::Map<&'static str, Symbol> = {};\n",
        builder.build()
    )
    .unwrap();
}
