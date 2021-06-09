use std::error::Error;
use std::fs::rename;
use std::path::Path;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rake",
    about = "Gather all leaves (files) in a nested directory structure and move them to the root"
)]
struct Cli {
    #[structopt(help = "The root directory you want to gather the nested leaves from")]
    dir: String,
}

// TODO: prompt user if number of files is high
// TODO: prompt user if dir is home dir
// TODO: prompt user by default, use flag to disable prompt
// TODO: add dryrun

fn main() -> Result<(), Box<dyn Error>> {
    let args: Cli = Cli::from_args();

    for entry in WalkDir::new(&args.dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let p_name = String::from(entry.path().to_string_lossy());

        let path = Path::new(&p_name);
        if path.is_file() {
            let new = format!("{}/{}", &args.dir, &f_name);
            println!("Renaming from {} to {}", &p_name, &new);
            rename(&p_name, &new).expect("Unable to rename");
        }
    }

    Ok(())
}
