# Port Sniffer CLI
Simple port sniffer made in rust as a learning project.
Please only use this for ethical purposes only!

# Usage
## Install via cargo
To install run:
```bash
cargo install portsniffer
```
To start portsniffer run:
```bash
portsniffer -h
```


## Build the project
To build it run:
```bash
cargo build --release
```
To start it run:
#### Linux:
```bash
./release/debug/portsniffer -s IPV4ADDRESS
```
#### Windows:
Open the folder target/debug and then execute portsniffer.exe

### To gather help run
```bash
./target/debug/portsniffer -h
```
or (if installed via cargo)
```bash
portsniffer -h
```