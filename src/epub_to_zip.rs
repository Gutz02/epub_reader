use xml_proc::tree_struct::Node;
use xml_proc::xml_proc::process_line_list;
use std::{fs, path::Path};
use std::io::{self, BufReader};
use zip::read::ZipArchive;


#[derive(Debug, PartialEq)]
pub enum MyError{
    FileNotFound,
    FailedToCreateFolder,
    FolderAlreadyExists,
    FailedToCreateZip
}

const PROCESSED_FOLDER_DIRECTORY : &str = "src\\processed_files";
const UNPROCESSED_FOLDER_DIRECTORY : &str = "src\\example_files";

pub fn read_epub(title: &String) -> Result<String, MyError> {
    let file_location = format!("{UNPROCESSED_FOLDER_DIRECTORY}\\{title}.epub");
    if !Path::new(&file_location).is_file(){
        return Err(MyError::FileNotFound);
    }

    let file_dir = format!("{PROCESSED_FOLDER_DIRECTORY}\\{title}");
    if folder_exists(&file_dir) {
        return Err(MyError::FolderAlreadyExists);
    }

    create_folder(&file_dir).map_err(|e| MyError::FailedToCreateFolder)?;
    Ok(file_dir)
}

pub fn unzip_epub(epub_path : &String, new_directory: &String) -> io::Result<()>{

    let extract_dir = new_directory;
    let file = fs::File::open(epub_path).unwrap();
    let reader = BufReader::new(file);
    let mut zip = ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let out_path = std::path::Path::new(extract_dir).join(file.name());

        if file.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // println!("Extracted: {}", out_path.display());
    }

    println!("EPUB extraction complete!");
    Ok(())
}

fn folder_exists(path: &String) -> bool {
    Path::new(path).is_dir()
}

fn create_folder(path: &String) -> Result<(), MyError> {
    match fs::create_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(MyError::FailedToCreateFolder),
    }
}

#[cfg(test)]
mod test_epub_to_zip{

    use super::*;

    fn delete_directory(path: &str) -> std::io::Result<()> {
        fs::remove_dir_all(path)?;
        println!("Deleted directory and all its contents: {}", path);
        Ok(())
    }
    
    #[test]
    fn test_file_not_found(){
        let title = "kate-chopin_short.epub".to_string();
        let error = read_epub(&title);
        assert_eq!(error, Err(MyError::FileNotFound));
    }

    #[test]
    fn test_epub_folder_created(){
        delete_directory("src\\processed_files\\kate-chopin_short-fiction");
        let title = "kate-chopin_short-fiction".to_string();
        read_epub(&title);
        assert!(folder_exists(&"src\\processed_files\\kate-chopin_short-fiction".to_string()));
    }

    #[test]
    fn test_epub_folder_already_exist(){
        let folder_name = "kate-chopin_short-fiction".to_string();
        let error = read_epub(&folder_name);
        assert_eq!(error, Err(MyError::FolderAlreadyExists));
    }

    #[test]
    fn zip_file_created(){
        delete_directory( "src\\processed_files\\kate-chopin_short-fiction");

        let folder_name = "src\\example_files\\kate-chopin_short-fiction.epub".to_string();
        let zip_location = "src\\processed_files\\kate-chopin_short-fiction".to_string();
        let some_err = unzip_epub(&folder_name, &zip_location);
        let file_count = fs::read_dir(zip_location)
        .expect("Failed to read directory")
        .count();
        assert_eq!(file_count, 3, "The number of extracted files is not 3");
        
    }

}