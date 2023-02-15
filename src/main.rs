mod core;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        eprintln!("Error: You need to specify a command to run. Run -h or --help for more information.");
        std::process::exit(1);
    } 

    let command: String = args[1].clone();

    if command == "-h" || command == "--help" {
        println!("Usage: {} [command] [arguments]", args[0]);
        println!(" -h, --help\tDisplay this help message");
        println!(" -v, --version\tDisplay the version of this program");
        println!(" -s, --scan\tScan an IP address. Usage: -s [IPv4 Address]");
    } else if command == "-v" || command == "--version" {
        let version: String = env!("CARGO_PKG_VERSION").to_string();
        println!("Version: {}", version);
    } else if command == "-s" || command == "--scan" {
        if args.len() == 2 {
            println!("Usage: {} [command] [arguments]", args[0]);
            println!(" -h, --help\tDisplay this help message");
            println!(" -v, --version\tDisplay the version of this program");
            println!(" -s, --scan\tScan an IP address. Usage: -s [IPv4 Address]");
            std::process::exit(1);
        }

        let ip: String = args[2].clone();
        core::scan_ip::scan(&ip);
    } else {
        println!("Usage: {} [command] [arguments]", args[0]);
        println!(" -h, --help\tDisplay this help message");
        println!(" -v, --version\tDisplay the version of this program");
        println!(" -s, --scan\tScan an IP address. Usage: -s [IPv4 Address]");
        std::process::exit(1);
    }

    std::process::exit(0);
}