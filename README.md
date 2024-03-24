![windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
[![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/rust-lang/rust)

[![linkedin](https://img.shields.io/badge/linkedin-0A66C2?style=for-the-badge&logo=linkedin&logoColor=white)](www.linkedin.com/in/joshua-kaplan-a88315245)
[![github](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/JKaps11)

![github](https://img.shields.io/github/followers/JKaps11.svg?style=social&label=Follow&maxAge=2592000)
# CLI App Launcher

A cli tool that allows you to store shortcuts to executables and run them by entering a custom keyword into you terminal.

Built in Rust using clap and serde-json crates

Works on Windows 10 and 11

## Usage/Examples

### *Important*
In order to add an executable, you need to either add an executable (.exe) or a shortcut (.lnk) to the directory /executables_dir, which is built when you run the program for the first time
### Launching One Program

#### Add an executable file with the keyword

```bash
  cal add <executable name> <keyword>
```

```bash
  cal add "Google Chrome.lnk" gc
```

#### Launch an executable file by keyword

```bash
  cal launch <keyword>
```

```bash
  cal launch gc
```

#### Remove an executable file by keyword

```bash
  cal remove <keyword>
```

```bash
  cal remove gc
```

### Launching Two or More Programs

#### Adding a Configuration

```bash
    cal config  <name> <executables list>
```

```bash
    cal add "Steam.lnk" st
    cal add "Google Chrome.lnk" gc
    cal config  start1 gc,st
```

#### Launching a Configuration

```bash 
    cal launch -c <keyword>
```


```bash 
    cal launch -c start1
```

#### Removing a Configuration

```bash
    cal remove -c <keyword>
```

```bash
    cal remove -c start1
```

### Looking at Create Configurations

#### Executables Added

```bash
    cal ls -e
```

#### Configurations Created

```bash
    cal ls -e
```

#### Both

```bash
    cal ls
```



## Installation

Download the [zip file](https://github.com/JKaps11/App-Launcher/releases/latest)

Setup a PATH environment variable leading to cal.exe

Try cal in the terminal of your choice

```bash
cal
```
This will let you know if your path is set up correctly and it will initialize the /executables_dir where you can store shortcuts and executable files for the cli tool to run
## Recommended Use

Set Terminal to launch on startup, so you can launch applications through the terminal 
without having to use your mouse.