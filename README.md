# Fritz Jingle Maschine

This program randomly plays the famous jingles of the [Fritz radio station](https://www.fritz.de) aired in Berlin and Brandenburg, Germany.
It is meant to be run on a Raspberry Pi (or similar mini-computer) that has a button attached to it's GPIOs. A button press triggers the playback of a randomly chosen jingle.

## Structure

The project is splitted into three parts:

* `fritz-jingle-maschine` - the program that randomly plays jingles
* `fritz-jingle-downloader` - a program that downloads or updates all the jingles from the fritz website
* `fritz-jingle-db` - a shared library for handling metadata about the jingles in a JSON file

The parts of the project are split into packages because the compilation on an old Raspberry Pi takes a looooooooong time (one of the reasons for this project was giving an old Raspberry Pi 1 a new purpose). 

Because of that, the code for the `maschine`-part is kept to a minimum (I wasn't able to build a working container for compilation with [`cross`](https://github.com/rust-embedded/cross)).

## Hardware

* Raspberry Pi
* Push-button for triggering the playback. The pin is configured as pull-down.
* 3.3V compatible LED to indicate readiness (optional)

## Build

For building the parts, go to the root of this repo and execute the corresponding commands.

### `fritz-jingle-downloader`

``` bash
cargo build --release --package fritz-jingle-downloader
```

### `fritz-jingle-maschine`

This is meant to be executed on the target device (the Raspberry Pi).
For the sound to play back smoothly, the build *must* be a `--release` target!

``` bash
cargo build --release --package fritz-jingle-maschine
```

## Run

You will find the executables in: `<PROJECT_ROOT>/target/release`.

### `fritz-jingle-downloader`

It is recommended to download the jingles to your computer and copy them to the Pi.
The downloader creates the following file structure at the parameter given to `PATH` (in this case, the target folder is `jingles`):

```
jingles/
├── db.json
└── files
    ├── 1\ Harz\ 4\ Kinder.mp3
    ├── 25\ Jahre\ Fritz\ (Mix-Up).mp3
    ├── 3\ Megahits\ am\ Stück.mp3
    ├── 5\ Dinge.mp3
    ├── 5\ Kilo\ Freude.mp3
    ├── 6\ Monate.mp3
    ├── 666.mp3
    ├── 90\ Jahre\ S-Bahn.mp3
    ├── Abfolge.mp3
    ...
```

#### Command line options

```
fritz-jingle-downloader 0.2.2
Beh <b@kayuk.de>


USAGE:
    fritz-jingle-downloader --jingles-path <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --jingles-path <PATH>    Downloads or updates all the jingles from Fritz to a given path. If
                                 a db.json is found in the path, missing jingles are downloaded.
```

### `fritz-jingle-maschine`

The `maschine` takes the path created by the downloader. It reads the `db.json` created by the downloader.

#### Command line options

```
fritz-jingle-maschine 0.2.1
Beh <b@kayuk.de>


USAGE:
    fritz-jingle-maschine [OPTIONS] --files-path <FILES-PATH> --button <BUTTON-PIN>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --button <BUTTON-PIN>        Specifies the Raspberry Pi GPIO pin for the trigger button. BCM
                                     numbering is used.
    -f, --files-path <FILES-PATH>    Path to the Jingles files containing db.json and a folder
                                     called files containing the MP3s.
    -l, --led <LED-PIN>              Specifies the Raspberry Pi GPIO pin for the (optional) LED. BCM
                                     numbering is used.
```

##  Autostart

On a Pi with RaspberryOS you can use the systemd service file in this repo. Adjust the parameters to your needs.

## Disclaimer

I am not, in any kind, related to Fritz or rbb. This is meant as a fun project to emphasize the greatness of these jingles. Since Fritz claims on it's [jingles website](https://www.fritz.de/programm/jingles/), that the jingles are "Zum Anhören, Runterladen und Weiterverschicken." (engl. "for listening, downloading and redistributing"), a project like this one here should be no problem. The website [linkfang.org](https://de.linkfang.org/wiki/Creative_Commons) claims, that these jingles ar published under a CC-License.