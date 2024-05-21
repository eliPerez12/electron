


#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum Operation {
    NOOP,
    ADD,
    SUB
}

#[derive(Debug, Default)]
struct OperationArgs {
    read_from_accumalator: bool,
    write_to_register_a: bool,
}


#[derive(Debug)]
struct Instruction {
    operation: Operation,
    operation_args: OperationArgs,
    a: u8,
    b: u8,
}


fn operation_from_str(string: String) -> Result<(Operation, OperationArgs), ()> {
    let str = string.as_str();
    if let Ok(operation) = match_operation_name(str) {
        Ok((operation, OperationArgs::default()))
    } else if let Ok(operation) = match_operation_name(str.get(1..).unwrap()) {
        Ok((operation, operation_args_from_str(string).unwrap()))
    } else {
        Err(())
    }
}

fn operation_args_from_str(string: String) -> Result<OperationArgs, ()>{
    match string.get(0..1).unwrap() { // first character of word
        "U" => Ok(OperationArgs {
            read_from_accumalator: true,
            write_to_register_a: true,
        }),
        "S" => Ok(OperationArgs {
            read_from_accumalator: false,
            write_to_register_a: true,
        }),
        "X" => Ok(OperationArgs {
            read_from_accumalator: true,
            write_to_register_a: false,
        }),
        _ => Err(())
    }
}

fn match_operation_name(str: &str) -> Result<Operation, ()> {
    match str {
        "ADD" => Ok(Operation::ADD),
        "SUB" => Ok(Operation::SUB),
        "NOOP" | "NOP" => Ok(Operation::NOOP),
        _ => Err(())
    }
}   

fn parse_line(line: &str) -> Result<Instruction, ()> {
    let mut line = line.to_string();
    let comment = line.find(';');
    if let Some(comment) = comment{
        line.truncate(comment);
    }
    let mut words: Vec<String> = line.split_whitespace().map(|s|s.to_owned()).collect();
    let (operation, operation_args) = operation_from_str(words.first().unwrap().clone()).unwrap();
    words.remove(0); // Get rid of operation so only oprands remain
    dbg!(words);
    Ok(Instruction {
        operation,
        operation_args,
        a: 0,
        b: 0,
    })
}

fn main() {
    let mut file = std::fs::File::open("program.elt").unwrap();
    let mut buffer = String::new();
    std::io::Read::read_to_string(&mut file, &mut buffer).unwrap();
    let mut instructions = vec![];
    for (line_num, line) in buffer.lines().enumerate() {
        instructions.push(parse_line(line).unwrap());
    }
    for (line_num, instruction) in instructions.iter().enumerate() {
        println!("{line_num}: {:?} {} {}", instruction.operation, instruction.a, instruction.b);
    }
}
