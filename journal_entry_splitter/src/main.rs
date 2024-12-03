use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Start creating a Path to be later read. 
    let filepath = Path::new("your_monthly_entry.txt");
    let display = filepath.display();

    // Open the file(stored at filepath) in read-only mode 
    let mut file = match File::open(&filepath) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string 
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("file read successfully!"),
    };

    let entries = file_content.split("------ ENTRY ------");

    for entry in entries {
        if entry.is_empty(){
            continue;
        }
        println!("Entry:{}",entry);
        let entry_lines = entry.split("\n");
        let mut file_title = String::new();
        let mut file_date = String::new();
        for (index,line) in entry_lines.enumerate(){
            if entry.is_empty(){
                continue;
            }
            let line_split: Vec<&str> = line.split(":").collect(); 
            if index == 1{
                // It consumes the title from String entry
                file_title = line_split[1].to_string();
            }
            if index == 2{
                // It consumes the entry date from String
                file_date = line_split[1].to_string();                
            }
            if index == 3 || index ==4{
                // After these lines we can break the loop
                // because we have all needed for creating file title
                break;
            }
        }
        // Prepare file name to match the requirements for target platform
        let mut newfile_path = String::new();
        newfile_path.push_str("output/📘 ");
        file_date.push_str("_");
        newfile_path.push_str(&file_date);
        newfile_path.push_str(&file_title);
        newfile_path.push_str(".txt");

        // Write a new file with the correct entry file and content
        let newfile = File::create(newfile_path);
        match newfile.expect("could not create file").write_all(entry.as_bytes()){
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => println!("{} written successfully!", file_title)
       }

    }
}
