# Fritz Jingle Maschine

This program randomly plays the famous jingles of the [Fritz radio station](https://www.fritz.de) aired in Berlin and Brandenburg, Germany.
It is meant to be run on a Raspberry Pi (or similar mini-computer) that has a button attached to it's GPIOs. A button press triggers the playback of a randomly chosen jingle.

## Structure

The project is splitted into three parts:

* `fritz-jingle-maschine` - the program that randomly plays jingles
* `fritz-jingle-downloader` - a program that downloads or updates all the jingles from the fritz website
* `fritz-jingle-db` - a shared library for handling metadata about the jingles in a JSON file

The reason for the split was, that the compilation on an old Raspberry Pi is takes a looooooooong time (One of the reasons for this project was giving an old Raspberry Pi 1 a new purpose). To keep the amount of code for the Raspi at a minimum, the `maschine`-part has it's own workspace.

## Hardware

TODO

## Build

For building the parts, go to the root of this repo and execute the corresponding commands.

### `fritz-jingle-downloader`

``` bash
cargo build --package fritz-jingle-downloader
```

### `fritz-jingle-maschine`

``` bash
cargo build --package fritz-jingle-maschine
```

## Run

TODO 

## Disclaimer

I am not, in any kind, related to Fritz or rbb. This is meant as a fun project to emphasize the greatness of these jingles. Since Fritz claims on it's [jingles website](https://www.fritz.de/programm/jingles/), that the jingles are "Zum Anhören, Runterladen und Weiterverschicken." (engl. "for listening, downloading and redistributing"), so a project like this one here should be no problem. The website [linkfang.org](https://de.linkfang.org/wiki/Creative_Commons) claims, that these jingles ar published under a CC-License.