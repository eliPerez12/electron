#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum Operation {
    NOOP,
    IMM,
    ADD,
    SUB,
    OUT,
}

impl Operation {
    // What operands this intruction requires (A, B)
    pub fn needed_oprands(&self, args: &OperationArgs) -> (bool, bool) {
        match self {
            Operation::NOOP => (false, false),
            Operation::ADD | Operation::SUB => match args {
                OperationArgs::None => (true, true), // No prefix
                OperationArgs::S => (true, true),    // S
                OperationArgs::U => (true, true),    // U
                OperationArgs::X => (false, true),   // X
            },
            Operation::OUT | Operation::IMM => (true, true),
        }   
    }
}

#[derive(Debug)]
enum OperationArgs {
    None,   // No prefix
    S,      // S
    U,      // U
    X,      // X
}


#[derive(Debug)]
struct Instruction {
    operation: Operation,
    operation_args: OperationArgs,
    a: u8,
    b: u8,
}


fn operation_from_str(line: &Vec<String>) -> Result<(Operation, OperationArgs), String> {
    if line.is_empty() {
        return Ok((Operation::NOOP, OperationArgs::None));
    }
    let string = line.first().unwrap().clone();
    if let Ok(operation) = match_operation_name(&string) {
        Ok((operation, OperationArgs::None))
    } else if let Ok(operation) = match_operation_name(string.get(1..).unwrap()) {
        Ok((operation, operation_args_from_str(string).unwrap()))
    } else {
        Err(format!("\"{string}\" is not a valid instruction"))
    }
}

fn operation_args_from_str(string: String) -> Result<OperationArgs, ()>{
    match string.get(0..1).unwrap() { // first character of word
        "U" => Ok(OperationArgs::U),
        "S" => Ok(OperationArgs::S),
        "X" => Ok(OperationArgs::X),
        _ => Err(())
    }
}

fn match_operation_name(str: &str) -> Result<Operation, ()> {
    match str {
        "IMM" => Ok(Operation::IMM),
        "ADD" => Ok(Operation::ADD),
        "SUB" => Ok(Operation::SUB),
        "NOOP" | "NOP" => Ok(Operation::NOOP),
        "OUT" => Ok(Operation::OUT),
        _ => Err(())
    }
}   

fn parse_line(line: &str) -> Result<Instruction, String> {
    let mut line = line.to_string().to_ascii_uppercase()
    .replace(['R', '%', '#', '$'], "");
    let comment = line.find(';');
    if let Some(comment) = comment{
        line.truncate(comment);
    }
    let mut words: Vec<String> = line.split_whitespace().map(|s|s.to_owned()).collect();
    let (operation, operation_args) = operation_from_str(&words)?;
    let (a, b) = {(
        if operation.needed_oprands(&operation_args).0 {
            words.remove(0);
            let oprand = words.first().unwrap();
            if let Ok(a) = oprand.parse() {
                a
            } else {
                return Err(format!("\"{oprand}\" is not a valid oprand"))
            }
        } else {
            0
        },
        if operation.needed_oprands(&operation_args).1 {
            words.remove(0);
            let oprand = words.first().unwrap();
            if let Ok(a) = oprand.parse() {
                a
            } else {
                return Err(format!("\"{oprand} is not a valid oprand\""))
            }
        } else {
            0
        })
    };
    Ok(Instruction {
        operation,
        operation_args,
        a,
        b,
    })
}

fn main() {
    let file_name = "program.elt";
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut buffer = String::new();
    std::io::Read::read_to_string(&mut file, &mut buffer).unwrap();
    let mut errors: Vec<String> = vec![];
    let mut warnings: Vec<String> = vec![];
    let mut instructions = vec![];
    for line in buffer.lines() {
        if let Ok(instruction) = parse_line(line) {
            instructions.push(instruction)
        } else if let Err(error) = parse_line(line){
            errors.push(error)
        }
    }
    if instructions.len() > 32 {
        warnings.push(format!("Too many lines of instruction ({}/32)", instructions.len()))
    }
    for warning in warnings {
        println!("Warning: {warning}.");
    }
    if errors.is_empty() {
        for (line_num, instruction) in instructions.iter().enumerate() {
            println!("{line_num}:  {:?} {:?} {} {}", instruction.operation, instruction.operation_args, instruction.a, instruction.b);
        }
        println!("Successfully validated program.");
    } else {
        println!("Failed to compile {file_name}.");
        for error in errors {
            println!("Error: {error}.");
        }
    }
}
