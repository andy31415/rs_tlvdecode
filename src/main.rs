use clap::Parser;
use tag_length_value_stream::{Parser as TlvParser, Value};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long)]
    hex: String,
}

fn main() {
    let args = Args::parse();

    let result = hex::decode(args.hex);

    let result = match result {
        Err(e) => {
            println!("Error decoding hex string: {:?}", e);
            return;
        }
        Ok(v) => v,
    };

    let mut parser = TlvParser::new(result.as_slice());

    println!("===================== DECODED ITEMS =========================");

    let mut indent = 0;

    loop {
        let item = match parser.next() {
            None => break,
            Some(value) => value,
        };

        if matches!(item.value, Value::ContainerStart(_)) {
            indent += 1;
        }

        println!("{:indent$}{:?}", "", item, indent = (indent * 2));

        if item.value == Value::ContainerEnd {
            indent = indent.saturating_sub(1);
        }
    }

    if !parser.done() {
        println!("!!!! INCOMPLETE DATA !!!!");
    }

    println!("=========================== END =============================");
}
