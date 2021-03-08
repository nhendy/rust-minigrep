#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result {

    }

}

fn search(file_data:&str, query:&str) -> Vec<&str>{
    vec![]        
}

#[derive(Debug)]
pub struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}
impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        Ok(Config {
            query: &args[1],
            filename: &args[2],
        })
    }
}

pub fn parse_args(args: &[String]) -> Config {
    Config {
        query: &args[1],
        filename: &args[2],
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Config:{:?}", config);
    let file_data = std::fs::read_to_string(config.filename)?;
    Ok(())
}
