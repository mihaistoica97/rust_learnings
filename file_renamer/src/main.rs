// Rename files in a directory to a known format

use std::fs::{self, ReadDir};
use std::path::PathBuf;

fn find_season_and_episode(file_names: Vec<String>){
    //let usual_season_signifiers = ["s", "v", "season"];
    let file_name: Vec<char> = file_names[0].chars().collect();
    let mut season_num = String::new();
    for x in 0..file_name.len(){
        // Normally season is designated with a character or the word "Season" nearby.
        // Probably add in a failsafe in case it makes it through the whole thing without finding the season
        let mut is_season_section = String::new();
        if file_name[x].is_numeric() && (file_name[x - 1].is_alphabetic() || file_name[x - 2].is_alphabetic()){
            println!("Does this section include the season number: '{}{}{}{}{}' (Y/N)", file_name[x - 2], file_name[x - 1], file_name[x], file_name[x + 1], file_name[x + 2]);            
            std::io::stdin().read_line(&mut is_season_section).unwrap_or(1);
            if is_season_section.to_ascii_uppercase().trim() == "Y" {
                let mut has_been_numeric = false;
                for y in (-2i8..2).into_iter(){
                    let index: usize = u8::try_from(i8::try_from(x).unwrap() + y).unwrap().into();
                    if has_been_numeric == true && file_name[index].is_numeric() == false{
                        break;
                    } else if season_num.len() == 2{
                        //The Tonight Show has 70 "seasons" (As in it's ran for 70 years), so I don't think we need to do
                        //more than 2 digit seasons for another 20 something years
                        break;
                    }
                    else if file_name[index].is_numeric(){                        
                        season_num.push(file_name[index]);
                        has_been_numeric = true;
                    }
                }
                println!("{}", season_num);                
                break;
            }
        }
    }
} 

fn group_files(paths: ReadDir) -> Vec<String>{
    let mut files_grouped: Vec<Vec<String>> = Vec::new();
    for path in paths {
        //println!("Name: {}", path.unwrap().path().display())
        let full_filename = path.as_ref().unwrap().file_name().into_string().unwrap();
        let filename_split = full_filename.split(".").collect::<Vec<&str>>();
        if filename_split.len() > 1 {
           
            let full_filepath = path.as_ref().unwrap().file_name().into_string().unwrap();
            let mut is_in_files_grouped = false;
            for (i, file_extension) in files_grouped.iter().enumerate(){
                if file_extension.iter().any(|s| s.contains(&filename_split.last().unwrap().to_string())){
                    files_grouped[i].push(full_filepath.clone());
                    println!("HERE {}", full_filename);
                    is_in_files_grouped = true;
                    break
                } 
            }
            if is_in_files_grouped == false{
                files_grouped.push(vec![full_filepath]);
            }
        }
    }
    println!("{:?}", files_grouped);
    files_grouped.sort_by(|a, b| b.len().cmp(&a.len()));
    println!("{:?}", files_grouped);
    files_grouped[0].clone()
}

fn main() {
    let mut directory_path = String::new();
    let mut path = PathBuf::from(directory_path.trim());
    while path.is_dir() == false { 
        directory_path = String::new();      
        println!("GIVE ME A PATH");
        std::io::stdin().read_line(&mut directory_path).unwrap_or(1);
        println!("File path is: {}", directory_path.trim());
        path = PathBuf::from(directory_path.trim());
        
    }
       
    let paths = fs::read_dir(directory_path.trim()).unwrap();
    let file_names = group_files(paths);
    println!("{:?}", file_names);
    //Add in args to go seasonless
    find_season_and_episode(file_names);
}
