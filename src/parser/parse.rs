use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./src/parser/own_lisp.pest"]
pub struct NumberParser;

pub fn simple_examples() {
    let parse_ok = NumberParser::parse(Rule::num, "273");
    println!("{:?}", parse_ok);

    let parse_ok = NumberParser::parse(Rule::expr, "273.15");
    println!("{:?}", parse_ok);

    let parse_not_ok = NumberParser::parse(Rule::file, "273.15");
    println!("{:?}", parse_not_ok);

    let parse_not_ok = NumberParser::parse(Rule::num, "this is not a number");
    println!("{:?}", parse_not_ok);
    println!("");
}
