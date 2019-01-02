# XSD

An XML Schema Definition (XSD) parser.

## Usage

``` rust
use structopt::StructOpt;
use xsd::parser::{Elements, Parser};

#[derive(Debug, StructOpt)]
struct Config {
    /// Path to XSD file
    #[structopt(short = "p", long = "path")]
    file_path: String,
}

fn main() {
    let config = Config::from_args();

    let parser = Parser::new(config.file_path).unwrap();

    println!(
        "Complex types: {}",
        parser
            .elements
            .iter()
            .filter(|e| match e {
                Elements::ComplexType(_) => true,
                _ => false,
            }).count()
    );
    println!(
        "Simple types: {}",
        parser
            .elements
            .iter()
            .filter(|e| match e {
                Elements::SimpleType(_) => true,
                _ => false,
            }).count()
    );
}
```

## TODO

* Tests
* Replace `String` with `&str`
* Closer spec conformance
