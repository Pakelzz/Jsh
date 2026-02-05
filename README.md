# Jsh
Jsh is a CLI application used to display prayer schedules for Indonesia.

## How To Use 
Display prayer schedules with name of city
```
jsh malang
```

Display prayer schedules with TUI
```
jsh --list
```

Display prayer schedules with id of city (To search id use jsh --list)
```
jsh --id 1001
```

Also we can make default config to simply call the apps
```bash
jsh --id 1001 -d
jsh jakarta -d
jsh --list -d 
```

After we have default config we can just run `jsh`

To Display prayer schedules without spinner animation
```bash
jsh run -s 
jsh run --id 1001 -s 
```

## Installation
There is no installation yet ...
