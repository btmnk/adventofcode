pub struct ParsedInstruction {
    pub amount: usize,
    pub source: usize,
    pub target: usize,
}

pub fn parse_instruction(input: &str) -> ParsedInstruction {
    let parts = input.split(" ").collect::<Vec<&str>>();
    let amount = parts[1].parse::<usize>().unwrap();

    // subtract 1 to start at 0
    let source = parts[3].parse::<usize>().unwrap() - 1;
    let target = parts[5].parse::<usize>().unwrap() - 1;

    return ParsedInstruction {
        amount,
        source,
        target,
    };
}
