use::clap::{command, Arg, Command};
use::serde_json::to_string;
use::serde::{Deserialize, Serialize};
use::std::path::Path;
use::std::fs::{File, DirBuilder, OpenOptions};

const EXECUTABLES_FILE: &str = "../executables.json";
const EXECUTABLES_DIRECTORY: &str = "../executables_dir";

static CURRENT_FILE_DATA: FileData;

#[derive(Serialize, Deserialize)]
struct ExecutableData {
    name: String,
    keyword: String,
    num_times_opened: u16,
}

impl ExecutableData {
    fn new(executable_name: &str, executable_keyword: &str) -> ExecutableData{
        ExecutableData {
            name: executable_name.to_string(),
            keyword: executable_keyword.to_string(), 
            num_times_opened: 0,
        }
    }

    fn increment_num_times_opened(&mut self) {
        self.num_times_opened += 1;
    }
}


#[derive(Serialize, Deserialize)]
struct FileData {
    num_executables: u8,
    executables: Vec<ExecutableData>
}

impl FileData {
    fn new() -> FileData{
        FileData {
            num_executables: 0,
            executables: Vec::new(),
        }
    }

    fn get_file_data() -> FileData {
        File::open(EXECUTABLES_FILE)
    }

    fn add_executable(&mut self, executable_data: ExecutableData) {
        self.executables.push(executable_data);
    }

    fn increment_num_executables(&mut self) {
        self.num_executables += 1;
    }
}

// handles creation of the file containing the names of the executable files and their keywords
// checks to see if the files is already there before creation
pub fn create_executable_file() {
    match Path::new(EXECUTABLES_FILE).try_exists() {
        Ok(true) => CURRENT_FILE_DATA = FileData::get_file_data(),
        Ok(false) => create_file(),
        Err(_) => panic!("There was an error checking if the Exectuables json exists"),
    }
}

// handles 1st time run on machine. will create executable file and initialize new FileData data structure.
pub fn create_file() {
    match File::create(EXECUTABLES_FILE) {
        Ok(_) => {
            CURRENT_FILE_DATA = FileData::new();
        },
        Err(e) => panic!("Error {} with file creation", e),
    }
}

// handles creation of directory containing the shortcuts for the executables
// dirbuilder create automatically checks if the directory already exists
pub fn create_executable_directory() {
    match DirBuilder::new().create(EXECUTABLES_DIRECTORY) {
        Ok(_) => (),
        Err(_) => panic!("There was an error checking if the Executables directory exitsts"),
    }
}

// handles adding an executable to executables json and updating metadata
pub fn add_executable(exec: &str, keyword: &str) {
    match OpenOptions::new().write(true).open(EXECUTABLES_FILE) {
        Ok(file) => {
            let new_executable_instance = ExecutableData::new(exec, keyword);
            
        
        },
        Err(e) => panic!("Error with opening executables file in write mode {})", e),
    }
}

// handles removing an executable forom executables json and updating metadata
pub fn remove_executable(keyword: &str) {

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


    if let Some(add_args) = match_result.subcommand_matches("add") {
        let executable_to_add = add_args.get_one::<String>("executable").unwrap();
        let keyword_to_add = add_args.get_one::<String>("keyword").unwrap();
        add_executable(executable_to_add, keyword_to_add);
    }

    if let Some(launch_args) = match_result.subcommand_matches("launch") {
        let keyword_to_search = launch_args.get_one::<String>("keyword").unwrap();
        launch_executable(keyword_to_search);
    }

    if let Some(remove_args) = match_result.subcommand_matches("remove") {
        let keyword_to_search = remove_args.get_one::<String>("keyword").unwrap();
        remove_executable(keyword_to_search);
    }
}

