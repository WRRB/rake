use std::fs::rename;
use std::error::Error;
use walkdir::WalkDir;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rake", about= "Gather leaves")]
struct Cli {
    directory: String
}

// TODO: prompt user if number of files is high
// TODO: prompt user if dir is home dir
// TODO: prompt user by default, use flag to disable prompt

fn main() -> Result<(), Box<dyn Error>> {
    let args: Cli = Cli::from_args();
    

    for entry in WalkDir::new(&args.directory)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let p_name = String::from(entry.path().to_string_lossy());

        let new = format!("{}/{}", &args.directory, &f_name);
        println!("Renaming from {} to {}", &p_name, &new);
        rename(&p_name, &new).expect("Unable to rename");
    }

    Ok(())
}
