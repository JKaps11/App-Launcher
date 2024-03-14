use::clap::{command, Arg, Command};
use::serde_json::to_string;
use::serde::{Deserialize, Serialize};
use::std::path::Path;
use::std::fs::File;
use::std::fs::DirBuilder;

const EXECUTABLES_FILE: &str = "../executables.json";
const EXECUTABLES_DIRECTORY: &str = "../executables_dir";
static FILE_META_DATA: FileMetaData = FileMetaData{
    num_executables:0
};

#[derive(Serialize, Deserialize)]
struct ExecutableData {
    name: String,
    keyword: String,
    num_times_opened: u16,
}


#[derive(Serialize, Deserialize)]
struct FileMetaData {
    num_executables: u8,
}

// handles creation of the file containing the names of the executable files and their keywords
// checks to see if the files is already there before creation
pub fn create_executable_file() {
    match Path::new(EXECUTABLES_FILE).try_exists() {
        Ok(true) => (),
        Ok(false) => File::create(EXECUTABLES_FILE),
        Err(_) => panic!("There was an error checking if the Exectuables json exists");
    }
}

// handles creation of directory containing the shortcuts for the executables
// dirbuilder create automatically checks if the directory already exists
pub fn create_executable_directory() {
    match DirBuilder::new().create(EXECUTABLES_DIRECTORY) {
        Ok(_) => (),
        Err(_) => panic!("There was an error checking if the Executables directory exitsts");
    }
}

// handles adding an executable to executables json and updating metadata
pub fn add_executable(json_data: String) {
    
}

// handles removing an executable forom executables json and updating metadata
pub fn remove_executable() {

}

// handles launching executable by seraching through executables json and finding the correct
// executable. Then it looks for it int the executables directory and launches it
pub fn launch_executable(path: &str) {

}

fn main() {
    create_executable_file();
    create_executable_directory();

    // CLI setup using clap
    let match_result = command!()
        .about("A cli tool that allows you to launch executables by custom keyword")
        .subcommand(
            Command::new("add")
                .arg(
                    Arg::new("executable")
                    .help("a shortcut to an executable file")
                    .value_parser(clap::value_parser!(String))
                )
                .arg(
                    Arg::new("keyword")
                    .help("keyword used to access executable")
                    .value_parser(clap::value_parser!(String))
                )
        )
        .subcommand(
            Command::new("launch")
                .arg(
                    Arg::new("keyword")
                    .value_parser(clap::value_parser!(String))
                )
        )
        .subcommand(
            Command::new("remove")
                .arg(
                    Arg::new("keyword")
                    .value_parser(clap::value_parser!(String))
                )
        )
        .get_matches();

    let add_args = match_result.subcommand_matches("add").unwrap();
    let executable_to_add = add_args.get_one::<String>("executable").unwrap();
    let keyword_to_add = add_args.get_one::<String>("keyword").unwrap();
    
    match to_string(&ExecutableData{name: executable_to_add, keyword: keyword_to_add, num_times_opened: 0}) {
        Ok(data) => add_executable(data),
        Err(e) => panic!("Error creating json for executable {}", e),
    }


    let launch_args = match_result.subcommand_matches("launch").unwrap();
    let keyword_to_search = launch_args.get_one::<String>("keyword").unwrap();
    launch_executable(keyword_to_search);

    let remove_args = match_result.subcommand_matches("remove").unwrap();
    let keyword_to_search = remove_args.get_one::<String>("keyword").unwrap();
    remove_executable(keyword_to_search);

}

