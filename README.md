
# CLI App Launcher

A cli tool that allows you to store shortcuts to executables and run them by entering a custom keyword into you terminal.

Works on Windows 10 and 11

## Usage/Examples

#### *Important* 
  - You need to add the .exe/.lnk file to the directory executables_dir manually
  - You also need to have [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) installed

#### Add an executable file with the keyword

```bash
  Cargo run -- add <executable name> <keyword>
```

```bash
  Cargo run -- add "Google Chrome.lnk" gc
```

#### Launch an executable file by keyword

```bash
  Cargo run -- launch <keyword>
```

```bash
  Cargo run -- launch gc
```

#### Remove an executable file by keyword

```bash
  Cargo run -- remove <keyword>
```

```bash
  Cargo run -- remove gc
```


## Run Locally

Clone the project

```bash
  git clone https://github.com/JKaps11/App-Launcher.git
```

Go to the project directory

```bash
  cd App_Launcher
```

Install dependencies

```bash
  Cargo run  
```
