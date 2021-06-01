use std::env::args;
use std::fs::rename;
use std::error::Error;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let first_argument: String = args().nth(1).unwrap();

    for entry in WalkDir::new(&first_argument)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let p_name = String::from(entry.path().to_string_lossy());

        let new = format!("{}/{}", &first_argument, &f_name);
        println!("Renaming from {} to {}", &p_name, &new);
        rename(&p_name, &new).expect("Unable to rename");
    }

    Ok(())
}
