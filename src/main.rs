use clap::Parser;
use unsvg::Image;
use std::fs::File;
use plotters::prelude::*;
mod value;
mod bytecode;
mod lex;
mod parse;
mod vm;
/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}
pub struct coordinate{
    x: i32,
    y: i32,
    head_degree:i32,
    color: i32,
    draw: bool,
}


fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let image = Image::new(width, height);

    match image_path.extension().map(|s| s.to_str()).flatten() {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    let mut turtle = coordinate{x:height as i32/2,y:width as i32/2,head_degree:0,color:1,draw:false};
    let file = File::open(&file_path).unwrap(); //read file

    let mut root = BitMapBackend::new(&image_path, (height, width)).into_drawing_area();
    root.fill(&WHITE);

    let proto = parse::ParseProto::load(file);
    vm::ExeState::new(&mut turtle).execute(&proto,&mut turtle,&mut root);
    root.present().unwrap();

    Ok(())
}

