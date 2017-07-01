# Candlelight
A tiny command line utility for changing my screen brightness on linux.

## Motivation
My laptop's brightness buttons didn't work all the time, and I wanted to learn
[`clap`](kbknapp/clap-rs).

## Limitations
Candlelight currently assumes that `/sys/class/backlight/intel_backlight/brightness`
is the file where the current screen brightness is kept and also the file that
should be modified to change the brightness.

The limits on brightness are hard-coded between 1 and 7500, as that is what my
backlight supports.

## Usage
```
$ cndl -h
Candlelight 0.1.0
Chris Vittal <christopher.vittal@gmail.com>
A tiny utility to get/set the brightness of my laptop

USAGE:
    cndl [FLAGS] [INPUT]

FLAGS:
    -h, --help                Prints help information
    -M                        Sets brightness to maximum value
    -m                        Sets brightness to minimum value
    -p, --preview             Previews change. Does not change any settings
    -q, --current-settings    Displays current settings
    -V, --version             Prints version information

ARGS:
    <INPUT>    Sets brightness by either absolute or percentage,
               the valid range is between 1 to 7500 or 0% to 100%.
```
