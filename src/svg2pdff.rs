
use svg2pdf;
use std

pub fn mysvg2pdf(src_path:&str,dst_path:&str){
    let svg=std::fs::read_to_string(src_path).unwrap();
    let pdf=svg2pdf::convert_str(&svg,svg2pdf::Options::default()).unwrap();
    std::fs::write(dst_path,pdf).unwrap();
}

