pub fn parse_args(args: &Vec<String>) -> minigrep::Config {
    minigrep::Config::new(&args).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let config: minigrep::Config = parse_args(&args);
    return minigrep::run(config);
}
