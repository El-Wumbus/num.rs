mod functions;
use functions::*;
use structopt::StructOpt;
#[macro_use]
extern crate prettytable;
use prettytable::{Table, Row};

fn main()
{
    let mut table = Table::new();
    table.add_row(row!["ASCII", "Decimal", "Octal", "Hex", "binary"]);
    let opt = Opt::from_args();
    let mut inputvec:Vec<i64> = Vec::new();
    for input in opt.value {
       inputvec.push (  
        match InputVal::decide(input)
       {
        Ok(x) => match x {
            InputVal::HexadecimalNumber(x) => x,
            InputVal::OctalNumber(x) => x,
            InputVal::DecimalNumber(x) => x,
            InputVal::BinaryNumber(x) => x,
            InputVal::ASCIIChar(x) => x,
        },
        Err(x) => {
            eprintln!("Error: {}", x);
            continue;
        },
       }
    );
    }

    for inputval in inputvec 
    {
        let row:Row;
            let ascii:char;
            if inputval.to_string().is_ascii()
            {
                ascii = inputval as u8 as char;
            }
            else {ascii = '\0'}
        row = row![format!("{}",ascii), format!("{}", inputval), format!("0{:o}", inputval), format!("0x{:X}", inputval), format!("0b{:b}", inputval)];
        table.add_row(row);
    }
    table.printstd();
    
}

#[derive(StructOpt, Debug)]
#[structopt(name = "num.rs")]
struct Opt
{
    /// The value, can be hex (0x41), octal (0o101), decimal (65), an ASCII char (A), or a binary number (0b01000001)
    #[structopt(name = "value")]
    value: Vec<String>,
}
