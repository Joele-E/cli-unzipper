use clap::Parser;
use std::fs;
use std::io;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    // let args: Vec<_> = std::env::args().collect();
    let my_args = Cli::parse();

    println!("pattern: , path {:?}", my_args.path);

    // if args.len() < 2 {
    //     eprintln!("Incorrect use of the tool")
    // }

    let file_name = std::path::Path::new(&my_args.path);
    let file = fs::File::open(&file_name).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            println!("Extracting {} to {} ", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}
