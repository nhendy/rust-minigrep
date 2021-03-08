#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result (){
        let query= "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}

fn search<'a>(query:&'a str, file_data:&'a str) -> Vec<&'a str>{
    let  mut result = Vec::new();
    for line in file_data.lines() {
        if line.contains(query){
            result.push(line);
        }
    }
    result
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
    for line in search(&config.query, &file_data){
        println!("{}", line);
    }
    Ok(())
}
