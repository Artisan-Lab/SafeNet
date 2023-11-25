use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::Read;
use syn;
use walkdir::WalkDir;
use std::io::Write;

fn main() {
    let source_folder = "/root/rustsrc/ast/knowledge";
    let output_folder = "/root/rustsrc/ast/ast"; 
    let mut counter = 1; 

    std::fs::create_dir_all(output_folder).expect("Failed to create output directory");

    for entry in WalkDir::new(source_folder).follow_links(true) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".rs") {
                        let relative_path = entry.path().strip_prefix(&source_folder).unwrap();
                        let output_path = Path::new(output_folder).join(relative_path);
                        let output_parent = output_path.parent().unwrap();
                        std::fs::create_dir_all(output_parent).expect("Failed to create output directory structure");

                        let mut file = File::open(entry.path()).expect("Open file failed");
                        let mut content = String::new();
                        file.read_to_string(&mut content).expect("Read file failed");
                        let ast = syn::parse_file(&content).expect("AST failed");

                        let file_stem = entry.path().file_stem().and_then(|s| s.to_str()).unwrap_or("output");
                        let output_name = output_path.with_extension("ast");

                        println!("{}", output_name.display());
                        let mut output_file = File::create(&output_name).expect("Create file failed");
                        output_file.write_all(format!("{:#?}", ast).as_bytes()).expect("Write failed");
                    }
                }
            }
        }
    }
}
