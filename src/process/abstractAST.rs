use std::fs::File;
use std::path::Path;
use std::io::Read;
use syn;
use walkdir::WalkDir;
use std::io::Write;


fn main(){
    // let path_name = "/root/rustsrc/def/src/tester2.rs";
    // let mut file = File::open(Path::new(path_name))
    //     .expect("Open file failed");
    // let mut content = String::new();
    // file.read_to_string(&mut content);
    // let ast = syn::parse_file(&content)
    //     .expect("ast failed");
    // println!("{:#?}",ast);
    for entry in WalkDir::new("/root/rustsrc/..").follow_links(true) {
        let a = format!("{}",entry.unwrap().path().display());
        if a.contains(".rs"){
            let path_name = &a;
            let mut file = File::open(Path::new(path_name))
                .expect("Open file failed");
            let mut content = String::new();
            file.read_to_string(&mut content);
            let ast = syn::parse_file(&content)
                .expect("ast failed");
            let output_name = &path_name.replace("/", "_").replace(".rs", ".ast"); //生成文件名
            println!("{}", output_name);
            let mut file = std::fs::File::create(output_name).expect("create failed");
            file.write_all(format!("{:#?}",ast).as_bytes()).expect("write failed");
        }
    }



    // let path_name = "/root/rustsrc/def/src/tester2.rs";
    // let mut file = File::open(Path::new(path_name))
    //     .expect("Open file failed");
    // let mut content = String::new();
    // file.read_to_string(&mut content);
    // let ast = syn::parse_file(&content)
    //     .expect("ast failed");
    // println!("{:#?}",ast);
}
