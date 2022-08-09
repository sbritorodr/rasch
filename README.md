# Rasch
*Automatize your commands easily and blazingly fast™️*

<b style="color:#f00d">This is in alpha, it might not work properly.</b>

Rust CLI used to execute any terminal commands selecting multiple files inside a directory. It uses easy to remember filters and settings. *(rust + batch)*

Also, you can choose not to select files and only loop n times a command with `-l <NUMBER>`. This is
useful for something like repeating `"echo rash"` n times to a maximun of **255**.

## Installation
### Using cargo (multi-platform, but needs compiling)
`cargo install rasch`

## Usage
You can access this info anytime with `rasch -h` or `--help`.

`rasch [COMMAND] <OPTIONS>`

For example, if you want to convert all files to mp4 in ffmpeg:
`rs-batch "ffmpeg -i [i] -c:v libx264 [o]"`

Wether `[i]` is the input files and `[o]` is the output files.

### Options
- `-h` or `--help`: Displays output help.
- `-V` or `--Version`: Displays version info.
- `-v` or `--verbose`: Displays all diagnosis and steps useful for debugging or diagnostics.
- `-l` or `loop-times`: Disables file detection and loops any given number of times any command. Useful if you only want to spam a command to a maximum of 255 times. By default is disabled (`0`).
- `-p` or `--path`: Selects input path. By default is the current one used by the terminal.
- `-o` or `--output`: Select output folder.
- `-f` or `--file`: Select file extension. By default selects all files in the directory.
- `-k`or `keep-ext`: Keep extension files in the output files. Disabled by default so you can add the extension (`[o].ogg`) in the command.

## Build
1. Clone this repository: `git clone http://github.com/sbritorodr/rasch`
2. Run `cargo build -r`

## How to contribute
1. Clone this repository `git clone http://github.com/sbritorodr/rasch`
2. Switch to `nightly` branch.
3. Make any changes. Don't forget to compile it before pushing.
4. Push the repo to the nightly branch

From time to time, nightly will be merged to the stable branch.

## "Planned" future features
- [ ] Add more installation options (AUR, snap...)
- [ ] Allow multiple file extensions
- [ ] Use default rust std::process::Command instead of `execute` crate
- [ ] Recursive flag.