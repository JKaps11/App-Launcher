use ::serde::{Deserialize, Serialize};
use ::serde_json::{from_str, to_string_pretty};
use ::std::collections::HashMap;
use ::std::fs::{read_to_string, write, DirBuilder, File};
use ::std::path::Path;

const EXECUTABLES_FILE: &str = "./executables.json";
const EXECUTABLES_DIRECTORY: &str = "./executables_dir";

#[derive(Serialize, Deserialize, Clone)]
struct ExecutableData {
    name: String,
    keyword: String,
    num_times_opened: u16,
}

impl ExecutableData {
    fn new(executable_name: &str, executable_keyword: &str) -> ExecutableData {
        ExecutableData {
            name: executable_name.to_string(),
            keyword: executable_keyword.to_string(),
            num_times_opened: 0,
        }
    }

    fn increment_num_times_opened(&mut self) {
        self.num_times_opened += 1;
    }

    fn launch(&mut self) {
        // command to launch lnk file on windows 10: START <path>

        let exe_name = format!("\"{}\"", self.name);

        let mut cmd = std::process::Command::new("powershell");
        cmd.current_dir(EXECUTABLES_DIRECTORY);
        cmd.args(["START", &exe_name]);

        let status = cmd.status().expect("failed to execute process");

        println!("Process finished with: {status}");

        self.increment_num_times_opened();
    }
}

// handles creation of directory containing the shortcuts for the executables
// dirbuilder create automatically checks if the directory already exists
pub fn create_executable_directory() {
    if DirBuilder::new().create(EXECUTABLES_DIRECTORY).is_ok() {
        println!("Executables Directory initialized");
    };
}

#[derive(Serialize, Deserialize)]
struct Conifguration {
    name: String,
    executables: Vec<String>,
}

impl Conifguration {
    fn new(configuration_name: &str, executables_list: Vec<String>) -> Conifguration {
        Conifguration {
            name: configuration_name.to_string(),
            executables: executables_list,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileData {
    num_executables: u8,
    executables: HashMap<String, ExecutableData>,
    configurations: HashMap<String, Conifguration>,
}

impl FileData {
    pub fn new() -> FileData {
        FileData {
            num_executables: 0,
            executables: HashMap::new(),
            configurations: HashMap::new(),
        }
    }

    pub fn add_configuration(&mut self, name: &str, executables: Vec<String>) {
        let new_configuration = Conifguration::new(name, executables);
        match self
            .configurations
            .insert(name.to_string(), new_configuration)
        {
            None => println!("configuration {} added successfully", name),
            Some(old_configuration) => {
                let mut user_choice = String::new();
                println!("Other configuration found with that name. Would you like to replace it with your new configuration? (yes/no)");
                std::io::stdin().read_line(&mut user_choice).unwrap();

                match user_choice.as_str() {
                    "yes" => println!("Configuration added successfully"),
                    "no" => {
                        self.configurations
                            .insert(name.to_string(), old_configuration)
                            .unwrap();
                        println!("Configuration added successfully");
                    }
                    _ => panic!("That was not an option, process terminating"),
                };
            }
        };
    }

    fn get_file_data() -> FileData {
        match read_to_string(EXECUTABLES_FILE) {
            Ok(json_str) => match from_str::<FileData>(&json_str) {
                Ok(file_data) => file_data,
                Err(e) => panic!("Error when converting json to FileData {}", e),
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

    pub fn add_executable(&mut self, executable_name: &str, keyword: &str) {
        let new_executable_data = ExecutableData::new(executable_name, keyword);
        match self.executables.insert(
            new_executable_data.keyword.clone(),
            new_executable_data.clone(),
        ) {
            None => println!(
                "executable {} added successfully with keyword {}",
                new_executable_data.name, new_executable_data.keyword
            ),
            Some(old_exe_data) => println!(
                "keyword {} that pointed to executable {} now points to executable {}",
                new_executable_data.keyword, old_exe_data.name, new_executable_data.name
            ),
        };

        self.num_executables += 1;
    }

    pub fn remove_executable(&mut self, keyword: &str) {
        match self.executables.remove(keyword) {
            None => panic!("No executable with keyword {} was found", keyword),
            Some(exe_data) => println!(
                "Executable {} with keyword {} succesfully removed",
                exe_data.name, keyword
            ),
        };

        self.num_executables -= 1;
    }

    pub fn launch_executable(&mut self, keyword: &str) {
        match self.executables.get_mut(keyword) {
            None => panic!("No executable found for keyword {}", keyword),
            Some(exe_data) => {
                exe_data.launch();
            }
        };
    }

    pub fn launch_configuration(&mut self, name: &str) {
        match self.configurations.get(name) {
            None => panic!("No configutaions found with name {}", name),
            Some(configuration) => {
                for executable_keyword in &configuration.executables {
                    match self.executables.get_mut(executable_keyword) {    
                        None => panic!("No executable found for keyword {}", executable_keyword),
                        Some(exe_data) => {
                            exe_data.launch();
                        }           
                    };
                }
            }
        };
    }

    // handles saving FileData to json
    pub fn save_data_to_executable_json(&self) {
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
            Ok(false) => match File::create(EXECUTABLES_FILE) {
                Ok(_) => println!("Executable JSON file created"),
                Err(e) => panic!("Error {} with file creation", e),
            },
            Err(e) => panic!(
                "There was an error checking if the Exectuables json exists {}",
                e
            ),
        };
    }
}
