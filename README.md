# rstool

A CLI toolkit implemented in  Rust. This is my personal training project, more features are developing.

## Install

```
git clone https://github.com/rust-space/rstool.git
```

## Usage

### time: time-related toolkit 

```
$rstool time -h
rstool-time 
time toolkit

USAGE:
    rstool time [OPTIONS]

OPTIONS:
    -c, --convert                  convert time format
    -d, --datetime <datetime>      convert datetime to timestamp
    -h, --help                     Print help information
    -n, --now                      output current time
    -s, --standard <std>           [default: usual] [possible values: rfc2822, rfc3339, timestamp,
                                   usual]
    -t, --timestamp <timestamp>    convert timestamp to datetime

#examples:
$rstool time // output: 2023-05-12 19:13:54
$rstool time -n // output: 2023-05-12 19:13:54
$rstool time -s timestamp // output: 1683890184
$rstool time -s rfc2822 // output: Fri, 12 May 2023 19:16:55 +0800
$rstool time -s rfc3339 // output: 2023-05-12T19:17:18.207906+08:00
$rstool time -c // output: 2023-05-12 11:16:24, convert current timesttamp to "%Y-%m-%d %H:%M:%S"
$rstool -ct 1683890184 // output: 2023-05-12 11:16:24
$rstool time -ct 1683890184 -s rfc2822 // output: Fri, 12 May 2023 11:16:24 +0000
$rstool time -ct 1683890184 -s rfc3339 // output: 2023-05-12T11:16:24+00:00
$rstool time -cd '2023-05-12 11:16:2' // output: 1683890162
```
### ip: ip-related toolkit

```
$rstool ip -h
rstool-ip 
ip toolkit

USAGE:
    rstool ip [OPTIONS]

OPTIONS:
    -h, --help                Print help information
    -l, --local               output local IP addr
    -p, --public              output public IP addr
    -r, --resolve <domain>    resolve domain to IP addr

# examples:
$rstool ip // output: Local IP address: x.x.x.x
$rstool ip -l // output: Local IP address: x.x.x.x
$rstool ip -p // output: Public IP address: x.x.x.x
$rstool ip -r www.baidu.com // output: <www.baidu.com> IP address: 36.152.44.95
```
### other (todo)

## Contributing

## License

MIT © rust-space
