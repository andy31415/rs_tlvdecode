use clap::Parser;
use hex;
use tag_length_value_stream::{Parser as TlvParser, Record, Value};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long)]
    hex: String,
}

fn main() {
    let args = Args::parse();

    let result = hex::decode(args.hex);

    let result = match result  {
        Err(e) => {
            println!("Error decoding hex string: {:?}", e);
            return;
        },
        Ok(v) => v,
    };

    let parser = TlvParser::new(result.as_slice());

    println!("===================== DECODED ITEMS =========================");

    let mut indent = 0;

    for item in parser {
        if matches!(item, Record {tag: _, value: Value::ContainerStart(_)}) { 
            indent += 1;
        }
        println!("{:indent$}{:?}", "", item, indent=(indent * 2));

        if matches!(item, Record {tag: _, value: Value::ContainerEnd}) { 
            indent -= 1;
        }
    }

    println!("=========================== END =============================");
}
