use::clap::{command, Arg, Command, ArgAction};

mod file_management;

fn check_os(){
    if cfg!(not(target_os = "windows")) {
        panic!("This CLI tool only works on windows for now. Stay tuned for updates!")
    }
}

fn main() {

    check_os();

    let current_file_data: &mut file_management::FileData = &mut file_management::FileData::new();

    current_file_data.create_executable_file();
    file_management::create_executable_directory();

    // CLI setup using clap
    let match_result = command!()
        .about("A cli tool that allows you to launch executables by custom keyword")
        .subcommand_required(true)
        // adds an executable to json
        .subcommand(
            Command::new("add")
                .arg(
                    Arg::new("keyword")
                    .required(true)
                    .help("keyword used to access executable")
                    .value_parser(clap::value_parser!(String))
                )
                .arg( 
                    Arg::new("executable")
                    .required(true)
                    .help("a shortcut to an executable file")
                    .value_parser(clap::value_parser!(String))
                )
        )

        // launches a shortcut by keyword
        .subcommand(
            Command::new("launch")          
                .arg(
                    Arg::new("isConfiguration")
                    .short('c')
                    .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("keyword")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                )
        )

        // removes an executable from the json 
        .subcommand(
            Command::new("remove")

                .arg(
                    Arg::new("remove_configuration")
                    .short('c')
                    .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("keyword")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                )
        )

        // creates a configuration to launch multiple apps at one
        .subcommand(
            Command::new("config")
                .arg(
                    Arg::new("keyword")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                )

                .arg(
                    Arg::new("executable_keywords")
                        .value_delimiter(',')
                        .required(true)
                        .value_parser(clap::value_parser!(String))
                )
        )
        .subcommand(
            Command::new("settings")
        )
        .subcommand(
            Command::new("ls")
                .arg(
                    Arg::new("list_executables")
                        .short('e')
                        .action(ArgAction::SetTrue)
                        .conflicts_with("list_configurations")
                )
                .arg(
                    Arg::new("list_configurations")
                        .short('c')
                        .action(ArgAction::SetTrue)
                        .conflicts_with("list_executables")
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
        
        match launch_args.get_flag("isConfiguration") {
            true => current_file_data.launch_configuration(keyword_to_search),
            false => current_file_data.launch_executable(keyword_to_search),
        }
    }

    if let Some(remove_args) = match_result.subcommand_matches("remove") {
        let keyword_to_search = remove_args.get_one::<String>("keyword").unwrap();
        
        match remove_args.get_flag("remove_configuration") {
            true => current_file_data.remove_configuration(keyword_to_search),
            false => current_file_data.remove_executable(keyword_to_search),
        };
    }

    if let Some(config_args) = match_result.subcommand_matches("config") {
        let keyword_to_search = config_args.get_one::<String>("keyword").unwrap();
        let executable_keyword = config_args.get_many::<String>("executable_keywords").unwrap().cloned().collect();
        current_file_data.add_configuration(keyword_to_search, executable_keyword);
    }

    if let Some(ls_args) = match_result.subcommand_matches("ls") {
        
        if ls_args.get_flag("list_executables") || ls_args.get_flag("list_configurations") {
            match ls_args.get_flag("list_executables") {
                true => current_file_data.list_executables(),
                false => current_file_data.list_configurations(),
            }
        }
        else{
            current_file_data.list_executables();
            current_file_data.list_configurations();
        }
            
    }
    
    current_file_data.save_data_to_executable_json();
}

