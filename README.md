# rstool

A development toolset devloped in Rust. This is my personal training project, more features are developing.

## Install

```
git clone https://github.com/rust-space/rstool.git
```

## Usage

### time: time-related tools

```
# `-n`(--now): output current time. Can use `-s`(--standard) set time format.

rstool time // output: 2023-05-12 19:13:54
rstool time -n // output: 2023-05-12 19:13:54
rstool time -s timestamp // output: 1683890184
rstool time -s rfc2822 // output: Fri, 12 May 2023 19:16:55 +0800
rstool time -s rfc3339 // output: 2023-05-12T19:17:18.207906+08:00

# `-c`(--convert): convert time format. The `-d`(--datetime) means convert datetime string to timestamp; the `-t`(--timestamp) means convert timestamp to datetime string.

rstool time -c // output: 2023-05-12 11:16:24, convert current timesttamp to "%Y-%m-%d %H:%M:%S"
rstool -ct 1683890184 // output: 2023-05-12 11:16:24
rstool time -ct 1683890184 -s rfc2822 // output: Fri, 12 May 2023 11:16:24 +0000
rstool time -ct 1683890184 -s rfc3339 // output: 2023-05-12T11:16:24+00:00
rstool time -cd '2023-05-12 11:16:2' // output: 1683890162
```
### ip: ip-related tools (todo)
### other (todo)

## Contributing

## License

MIT © rust-space
