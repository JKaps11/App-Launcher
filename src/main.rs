use::clap::{command, Arg, Command};
use::serde_json::{to_string_pretty, from_str};
use::serde::{Deserialize, Serialize};
use::std::collections::HashMap;
use::std::path::Path;
use::std::fs::{File, DirBuilder, write, read_to_string};

const EXECUTABLES_FILE: &str = "../executables.json";
const EXECUTABLES_DIRECTORY: &str = "../executables_dir";


fn check_os(){
    if cfg!(not(target_os = "windows")) {
        panic!("This CLI tool only works on windows for now. Stay tuned for updates!")
    }
}

#[derive(Serialize, Deserialize, Clone)]
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
    executables: HashMap<String, ExecutableData>
}

impl FileData {
    fn new() -> FileData{
        FileData {
            num_executables: 0,
            executables: HashMap::new(),
        }
    }

    fn get_file_data() -> FileData {
        match read_to_string(EXECUTABLES_FILE){
            Ok(json_str) => {
                match from_str::<FileData>(&json_str){
                    Ok(file_data) => file_data,
                    Err(e) => panic!("Error when converting json to FileData {}", e),
                }
            },
            Err(e) => panic!("Error when getting json from executables file {}", e),
        }
    }

    fn to_json(&self) -> String {
        match to_string_pretty(self) {
            Ok(str) => str,
            Err(e) => panic!("Error when converting FileData to json string {}", e),
        }
    }

    fn add_executable(&mut self, executable_name: &str, keyword: &str) {
        let new_executable_data = ExecutableData::new(executable_name, keyword);
        match self.executables.insert(new_executable_data.keyword.clone(), new_executable_data.clone()){
            None => println!("executable {} added successfully with keyword {}", new_executable_data.name, new_executable_data.keyword),
            Some(old_exe_data) => println!("keyword {} that pointed to executable {} now points to executable {}", new_executable_data.keyword, old_exe_data.name, new_executable_data.name),
        };

        self.num_executables += 1;
    }
    
    fn remove_executable(&mut self, keyword: &str){
        match self.executables.remove(keyword) {
            None => panic!("No executable with keyword {} was found", keyword),
            Some(exe_data) => println!("Executable {} with keyword {} succesfully removed", exe_data.name, keyword),
        };

        self.num_executables -= 1;
    }

    fn launch_executable(&mut self, keyword: &str) {
       match self.executables.get_mut(keyword) {
            None => panic!("No executable found for keyword {}", keyword),
            Some(exe_data) => {
                exe_data.increment_num_times_opened();
                let executable_path = format!("{}/{}", EXECUTABLES_DIRECTORY, exe_data.name);
                // command to launch lnk file on windows 10: START <path>
                std::process::Command::new("START")
                    .arg(executable_path)
                    .status()
                    .expect("failed to launch executable");
            }
        };
    }

    // handles saving FileData to json
    fn save_data_to_executable_json(&self) {
        match write(EXECUTABLES_FILE, self.to_json()) {
            Ok(_) => println!("Your configuration was successfully saved!"),
            Err(e) => panic!("Unable to write FileData to File {}", e),
        };
    }


    // handles creation of the file containing the names of the executable files and their keywords
    // checks to see if the files is already there before creation
    pub fn create_executable_file(&mut self) {
        match Path::new(EXECUTABLES_FILE).try_exists() {
            // not first time running
            Ok(true) => *self = FileData::get_file_data(),
        
            //first time running
            Ok(false) => {
                match File::create(EXECUTABLES_FILE) {
                    Ok(_) => println!("Executable JSON file created"),
                    Err(e) => panic!("Error {} with file creation", e),
                }
            },
            Err(e) => panic!("There was an error checking if the Exectuables json exists {}", e),
        };
    }
}


// handles creation of directory containing the shortcuts for the executables
// dirbuilder create automatically checks if the directory already exists
pub fn create_executable_directory() {
    match DirBuilder::new().create(EXECUTABLES_DIRECTORY) {
        Ok(_) => println!("Executables Directory initialized"),
        Err(_) => panic!("There was an error checking if the Executables directory exitsts"),
    };
}

fn main() {

    check_os();

    let current_file_data: &mut FileData = &mut FileData::new();

    current_file_data.create_executable_file();
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
        current_file_data.add_executable(executable_to_add, keyword_to_add);
    }

    if let Some(launch_args) = match_result.subcommand_matches("launch") {
        let keyword_to_search = launch_args.get_one::<String>("keyword").unwrap();
        current_file_data.launch_executable(keyword_to_search);
    }

    if let Some(remove_args) = match_result.subcommand_matches("remove") {
        let keyword_to_search = remove_args.get_one::<String>("keyword").unwrap();
        current_file_data.remove_executable(keyword_to_search);
    }
    
    current_file_data.save_data_to_executable_json();
}

