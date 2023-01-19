# Vocal

## Description
Vocal is a terminal app to play music and to look cool while doing it.

## Installation

### Manual
```bash
git clone https://github.com/Jackboxx/vocal
cd vocal 
make 
make install
```

#### Issues
| error | fixes |
|:-----:|:-----:|
|`Package alsa was not found in pkg-config search path` | try installing the `alsa-lib-devel` package on Fedora or these packages `alsa-base libasound2 libasound2-dev libudev-dev` on Ubuntu. |

## Configuration

### Options 
- [color](#color)
- [highlight_color](#highlight_color)
- [audio_directory](#audio_directory)
- [starting_volume](#starting_volume)
- [starting_speed](#starting_speed)

### color 

##### Description
Color of the audio graph, progress bar, text, and list items.

##### Default Value
```toml
color = 'blue'
```

##### Valid Values
|  lower case  |  upper case  |
|:------------:|:------------:|
| black        | Black        |
| red          | Red          |
| green        | Green        |
| yellow       | Yellow       |
| blue         | Blue         |
| magenta      | Magenta      |
| cyan         | Cyan         |
| gray         | Gray         |
| lightred     | LightRed     |
| lightgreen   | LightGreen   |
| lightyellow  | LightYellow  |
| lightblue    | LightBlue    |
| lightmagenta | LightMagenta |
| lightcyan    | LightCyan    |
| white        | White        |

### highlight_color

##### Description
Color of the selected list item.

##### Default Value
```toml
highlight_color = 'magenta'
```

##### Valid Values
|  lower case  |  upper case  |
|:------------:|:------------:|
| black        | Black        |
| red          | Red          |
| green        | Green        |
| yellow       | Yellow       |
| blue         | Blue         |
| magenta      | Magenta      |
| cyan         | Cyan         |
| gray         | Gray         |
| lightred     | LightRed     |
| lightgreen   | LightGreen   |
| lightyellow  | LightYellow  |
| lightblue    | LightBlue    |
| lightmagenta | LightMagenta |
| lightcyan    | LightCyan    |
| white        | White        |

### audio_directory

##### Description
The directory that is loaded if no values are provided to the `--load` or `--play` flags.
All files in this directory are listed and can be selected to be played.

##### Default Value
```toml
audio_directory = '$HOME/vocal'
```

### starting_volume

##### Description
The volume level that audio will play at after the application is first started.

##### Default Value
```toml
starting_volume = 50
```

##### Max
```toml
starting_volume = 100
```

##### Min
```toml
starting_volume = 0
```

### starting_speed

##### Description
The speed that audio will play at after the application is first started.

##### Default Value
```toml
starting_speed = 100
```

##### Max
```toml
starting_speed = 200
```

##### Min
```toml
starting_speed = 10
```
