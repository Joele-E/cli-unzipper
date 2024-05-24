use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    path: PathBuf,
    outpath: Option<PathBuf>,
}

fn main() {
    let my_args = Cli::parse();

    let file_name = std::path::Path::new(&my_args.path);
    let file = fs::File::open(&file_name).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match &my_args.outpath {
            Some(p) => {
                let mut mypath = PathBuf::from(p);
                mypath.push(file.enclosed_name().unwrap());
                mypath
            }
            None => match file.enclosed_name() {
                Some(path) => path.to_path_buf(),
                None => continue,
            },
        };
        println!("Extracting {} to {} ", file.name(), outpath.display());
        if (*file.name()).ends_with('/') {
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
    println!("Completed")
}
