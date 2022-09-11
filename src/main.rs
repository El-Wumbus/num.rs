mod functions;
use functions::*;
use structopt::StructOpt;
extern crate prettytable;
use prettytable::{Table, Row};

fn main()
{
    let mut table = Table::new();
    let inputvec: Vec<i64> = parse_args(Opt::from_args());
    run(&mut table, inputvec);
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

fn parse_args(opt: Opt) -> Vec<i64>
{
    let mut inputvec: Vec<i64> = Vec::new();
    for input in opt.value
    {
        inputvec.push(match InputVal::decide(input)
        {
            Ok(x) => match x
            {
                InputVal::HexadecimalNumber(x) => x,
                InputVal::OctalNumber(x) => x,
                InputVal::DecimalNumber(x) => x,
                InputVal::BinaryNumber(x) => x,
                InputVal::ASCIIChar(x) => x,
            },
            Err(x) =>
            {
                eprintln!("Error: {}", x);
                continue;
            }
        });
    }
    inputvec.dedup();
    inputvec
}

fn run(table: &mut Table, inputvec: Vec<i64>)
{
    // First Row
    let mut first_row_list: Vec<&str> = Vec::new();
    let mut ascii: bool = false;
    for input in inputvec.clone()
    {
        if input > 32 && input < 127
        {
            ascii = true;
        }
    }

    if ascii
    {
        first_row_list.push("ASCII");
    }

    first_row_list.append(&mut vec!["Decimal", "Octal", "Hex", "Binary"]);
    table.add_row(Row::from(first_row_list.clone()));

    // Actual Info
    for inputval in inputvec
    {
        let row: Row;
        let ascii: char;

        if inputval < 256
        {
            ascii = inputval as u8 as char;
        }
        else
        {
            ascii = '\0'
        }
        let mut rowlist: Vec<String> = Vec::new();
        for item in first_row_list.clone()
        {
            match item
            {
                "ASCII" => rowlist.push(format!("{}", ascii)),
                "Decimal" => rowlist.push(format!("{}", inputval)),
                "Octal" => rowlist.push(format!("0{:o}", inputval)),
                "Hex" => rowlist.push(format!("0x{:X}", inputval)),
                "Binary" => rowlist.push(format!("0b{:b}", inputval)),
                &_ => (),
            }
        }
        row = Row::from(rowlist);
        table.add_row(row);
    }
}
