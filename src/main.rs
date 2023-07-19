use clap::Parser;
use tag_length_value_stream::{ParseResult, Parser as TlvParser, Value, Record};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long)]
    hex: String,
}

fn main() {
    let result = match hex::decode(Args::parse().hex) {
        Err(e) => {
            println!("Error decoding hex string: {:?}", e);
            return;
        }
        Ok(v) => v,
    };


    println!("===================== DECODED ITEMS =========================");

    let mut indent = 0;
    
    for item in TlvParser::new(result.as_slice()) {
        if matches!(item, ParseResult::Record(Record {tag: _, value: Value::ContainerStart(_)})) {
            indent += 1;
        }
        
        match item {
            ParseResult::Record(r) => println!("{:indent$}{:?}", "", r, indent = (indent * 2)),
            ParseResult::Error(e) => println!("!!!! PARSE ERROR: {:?}", e),
        }


        if matches!(item, ParseResult::Record(Record {tag: _, value: Value::ContainerEnd})) {
            indent = indent.saturating_sub(1);
        }
    }

    println!("=========================== END =============================");
}
