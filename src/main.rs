use std::process::Command;
use regex::Regex;
use std::collections::HashMap;
use std::env;

fn print_help() {
    println!("Displays network connections with owner process names.\n");
    println!("NETTL [-t] [-u] [-l]\n");
    println!("  -t            Display TCP connections.");
    println!("  -u            Display UDP connections.");
    println!("  -l            Display LISTENING connections.\n");
}

fn get_tasklist() -> HashMap<String, String> {
    let output = Command::new("tasklist").output().unwrap();
    let pattern = Regex::new(r"(.+?) +([0-9]+).+\n").unwrap();
    let data = String::from_utf8_lossy(&output.stdout);
    let mut tasks = HashMap::<String, String>::new();

    for cap in pattern.captures_iter(&data) {
        tasks.insert(String::from(&cap[2]), String::from(&cap[1]));
    }

    tasks
}

fn get_netstat() -> Vec::<(String, String, String, String, String)> {
    let output = Command::new("netstat").arg("-a").arg("-n").arg("-o").output().unwrap();
    let pattern = Regex::new(r" +?([A-Z]+) +?([[a-z][0-9]\.%:\[\]\*]+) +?([[a-z][0-9]\.%:\[\]\*]+) +?([A-Z]*) +?([0-9]+).+?\n").unwrap();
    let data = String::from_utf8_lossy(&output.stdout);
    let mut nets = Vec::<(String, String, String, String, String)>::new();

    for cap in pattern.captures_iter(&data) {
        nets.push((String::from(&cap[1]), String::from(&cap[2]), String::from(&cap[3]), String::from(&cap[4]), String::from(&cap[5])));
    }

    nets
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut tcp = false;
    let mut udp = false;
    let mut listening = false;
    let mut help = false;

    if args.len() == 1 {
        help = true;
    }

    // Remove executable name from the argument list
    args.remove(0);

    for arg in args {
        match arg.as_str() {
            "-t" => tcp = true,
            "-u" => udp = true,
            "-l" => listening = true,
            _    => help = true,
        }
    }

    if help == true {
        print_help();
        return
    }

    println!("Netstat meets Tasklist\n");
    println!("{:17} {:21} {:21} {:11} {:3}", "Process", "Local Address", "Foreign Address", "State", "Proto");

    let netstat = get_netstat();
    let tasklist = get_tasklist();

    for net in netstat {
        if (net.0 == "TCP" && tcp == true) ||
           (net.0 == "UDP" && udp == true) ||
           (net.3 == "LISTENING" && listening == true) {
               println!("{:17} {:21} {:21} {:11} {:3}", tasklist[&net.4], net.1, net.2, net.3, net.0);
        }
    }
}
