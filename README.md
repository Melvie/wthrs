# wthrs
Commandline weather app in Rust

Queries weather data from Weatherbit and displays on command line (Requires Weatherbit API key).
Default commands & api keys stored with confy.

## Usage
```bash
wthrs 

wthrs 0.1.0

USAGE:
    wthrs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    config      
    forecast    
    help        Prints this message or the help of the given subcommand(s)
```
### Setting config

```bash
wthrs config -h city_name -b country_name -k api_key -d num_of_days_to_forecast
```

### Getting a Forecast

``` bash
wthrs forecast

#################################################

Today's weather for Vancouver,CA: 
Few clouds 🌤
Current Temp: 12.2
Feels like: 11.450001
High/Low: 14.3/8.5
POP%: 0

Forecast for next: 3 day(s)
------------------------------------
Date: 2021-05-10
Scattered clouds 🌥
Current Temp: 11.9
Feels like: 11.9
High/Low: 15.3/9.4
POP%: 0
------------------------------------
Date: 2021-05-11
Scattered clouds 🌥
Current Temp: 13.1
Feels like: 12.95
High/Low: 16.5/10.1
POP%: 0

#################################################
```


