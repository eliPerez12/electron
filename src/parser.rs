pub struct ProgramLoader;

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl ProgramLoader {
    pub fn load_program(file_name: &str) -> Program {
        if file_name.is_empty() {
            println!("Error: No file name given. This argument is required: \"-f your_file_name\".");
            std::process::exit(1);
        }
        let mut file = match std::fs::File::open(file_name) {
            Ok(file) => file,
            Err(_) => {
                println!("Error: Could not find file specified. {file_name}");
                std::process::exit(1);
            },
        };
        let mut buffer = String::new();
        std::io::Read::read_to_string(&mut file, &mut buffer).unwrap();
        let mut errors: Vec<CompileMessage> = vec![];
        let mut warnings: Vec<CompileMessage> = vec![];
        let mut instructions = vec![];
        for (line_num, line) in buffer.lines().enumerate() {
            if let Ok(instruction) = parse_line(line) {
                instructions.push(instruction)
            } else if let Err(error) = parse_line(line) {
                errors.push(CompileMessage {
                    line: line_num,
                    message: error,
                })
            }
        }
        add_warnings(&instructions, &mut warnings);
        for warning in warnings {
            println!("Warning on line {}: {}.", warning.line, warning.message);
        }
        // Successfull Validation
        if errors.is_empty() {
            for (line_num, instruction) in instructions.iter().enumerate() {
                println!(
                    "{line_num}:  {:?} {:?} {:?} {:?}",
                    instruction.operation, instruction.operation_args, instruction.a, instruction.b
                );
            }
            // Add empty lines if lines < 32
            for _ in 0..32 - instructions.len().min(32) {
                instructions.push(Instruction::none())
            }
            println!("Successfully validated program.");
            Program { instructions }
        // Failed to validate
        } else {
            for error in errors {
                println!("Error on line {}: {}.", error.line, error.message);
            }
            println!("Failed to compile {file_name}.");
            std::process::exit(1)
        }
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum Operation {
    // Data 
    NOOP,
    IMM,
    MOV,

    // Alu
    ADD,
    ADDC,
    SHR,
    NOT,

    // IO
    OUT,

    // Control
    JMP,
    BIE,
}

impl Operation {
    pub fn get_name(&self) -> String {
        match self {
            Operation::NOOP => "|         ".to_string(),
            Operation::IMM => "| IMM     ".to_string(),
            Operation::MOV => "| MOV     ".to_string(),
            Operation::ADD => "| ADD     ".to_string(),
            Operation::ADDC => "| ADDC    ".to_string(),
            Operation::OUT => "| OUT     ".to_string(),
            Operation::JMP => "| JMP     ".to_string(),
            Operation::BIE => "| BIE     ".to_string(),
            Operation::SHR => "| SHR     ".to_string(),
            Operation::NOT => "| NOT     ".to_string(),
        }
    }
}

struct CompileMessage {
    line: usize,
    message: String,
}

impl Operation {
    // What operands this intruction requires (A, B)
    pub fn needed_oprands(&self, args: &OperationArgs) -> (Option<Oprand>, Option<Oprand>) {
        match self {
            Operation::NOOP => (None, None),
            Operation::IMM => (Some(Oprand::Register(0)), Some(Oprand::Immediate(0))),
            Operation::MOV => (Some(Oprand::Register(0)), Some(Oprand::Register(0))),
            Operation::ADD | Operation::ADDC => match args {
                OperationArgs::None => (Some(Oprand::Register(0)), Some(Oprand::Register(0))), // No prefix
                OperationArgs::S => (Some(Oprand::Register(0)), Some(Oprand::Register(0))),    // S
                OperationArgs::U => (Some(Oprand::Register(0)), Some(Oprand::Register(0))),    // U
                OperationArgs::X => (None, Some(Oprand::Register(0))),                         // X
            },
            Operation::SHR => (Some(Oprand::Register(0)), Some(Oprand::Register(0))),
            Operation::NOT => (Some(Oprand::Register(0)), Some(Oprand::Register(0))),
            Operation::OUT => (Some(Oprand::Port(0)), Some(Oprand::Register(0))),
            Operation::JMP | Operation::BIE =>(Some(Oprand::Immediate(0)), None),
            
        }
    }

    pub fn is_alu_operation(&self) -> bool {
        matches!(self, Operation::ADD | Operation::ADDC)
    }
}

#[derive(Debug, Clone)]
pub enum OperationArgs {
    None, // No prefix
    S,    // S
    U,    // U
    X,    // X
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub operation_args: OperationArgs,
    pub a: Oprand,
    pub b: Oprand,
}

impl Instruction {
    pub fn none() -> Self {
        Self {
            operation: Operation::NOOP,
            operation_args: OperationArgs::None,
            a: Oprand::Immediate(0),
            b: Oprand::Immediate(0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Oprand {
    Register(u8),
    MemoryAddress(u8),
    Immediate(u8),
    Port(u8),
}

impl Oprand {
    fn get_oprand_name(&self) -> String {
        match self {
            Oprand::Register(_) => "Register".to_string(),
            Oprand::MemoryAddress(_) => "MemoryAddress".to_string(),
            Oprand::Immediate(_) => "Immediate".to_string(),
            Oprand::Port(_) => "Port".to_string(),
        }
    }

    pub fn data(&self) -> u8 {
        *match self {
            Oprand::Register(data) => data,
            Oprand::MemoryAddress(data) => data,
            Oprand::Immediate(data) => data,
            Oprand::Port(data) => data,
        }
    }
}

fn operation_from_str(line: &Vec<String>) -> Result<(Operation, OperationArgs), String> {
    if line.is_empty() {
        return Ok((Operation::NOOP, OperationArgs::None));
    }
    let string = line.first().unwrap().clone();
    if let Ok(operation) = match_operation_name(&string) {
        Ok((operation, OperationArgs::None))
    } else if let Ok(operation) = match_operation_name(string.get(1..).unwrap()) {
        if operation.is_alu_operation() {
            Ok((operation, operation_args_from_str(string).unwrap()))
        } else {
            Err(format!("\"{:?}\" does not take ALU arguments", operation))
        }
    } else {
        Err(format!("\"{string}\" is not a valid instruction"))
    }
}

fn operation_args_from_str(string: String) -> Result<OperationArgs, ()> {
    match string.get(0..1).unwrap() {
        // first character of word
        "U" => Ok(OperationArgs::U),
        "S" => Ok(OperationArgs::S),
        "X" => Ok(OperationArgs::X),
        _ => Err(()),
    }
}

fn match_operation_name(str: &str) -> Result<Operation, ()> {
    match str {
        "IMM" => Ok(Operation::IMM),
        "MOV" => Ok(Operation::MOV),
        "ADD" => Ok(Operation::ADD),
        "ADDC" => Ok(Operation::ADDC),
        "SHR" => Ok(Operation::SHR),
        "NOOP" | "NOP" => Ok(Operation::NOOP),
        "OUT" => Ok(Operation::OUT),
        "JMP" => Ok(Operation::JMP),
        "BIE" => Ok(Operation::BIE),
        "NOT" => Ok(Operation::NOT),
        _ => Err(()),
    }
}

fn parse_oprand_binary(str: &str) -> Result<u8, String> {
    let string = str.to_string().replace('_', "");
    if string.get(0..1).unwrap() == "B" {
        let binary = string.get(1..).unwrap();
        let mut total = 0;
        for (place, char) in binary.chars().rev().enumerate() {
            total += char.to_string().parse::<u8>().unwrap() * 2u8.pow(place as u32);
        }
        Ok(total)
    } else {
        Ok(string.parse().unwrap())
    }
}

fn parse_oprand(oprand: &str) -> Result<Oprand, String> {
    let prefix = oprand.get(0..1);
    if let Ok(a) = oprand.parse() {
        Ok(Oprand::Immediate(a))
    } else if prefix.unwrap() == "R" {
        Ok(Oprand::Register(parse_oprand_binary(oprand.get(1..).unwrap()).unwrap()))
    } else if prefix.unwrap() == "#" {
        Ok(Oprand::MemoryAddress(
            parse_oprand_binary(oprand.get(1..).unwrap()).unwrap(),
        ))
    } else if prefix.unwrap() == "%" {
        Ok(Oprand::Port(parse_oprand_binary(oprand.get(1..).unwrap()).unwrap()))
    } else if let Ok(a) = parse_oprand_binary(oprand) {
        Ok(Oprand::Immediate(a))
    }
    else {
        return Err(format!("\"{oprand}\" is not a valid oprand"));
    }
}

fn parse_line(line: &str) -> Result<Instruction, String> {
    let mut line = line.to_string().to_ascii_uppercase();
    let comment = line.find(';');
    if let Some(comment) = comment {
        line.truncate(comment);
    }
    let mut words: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
    let (operation, operation_args) = operation_from_str(&words)?;
    let (a, b) = {
        (
            if operation.needed_oprands(&operation_args).0.is_some() {
                words.remove(0);
                parse_oprand(words.first().unwrap())?
            } else {
                Oprand::Immediate(0)
            },
            if operation.needed_oprands(&operation_args).1.is_some() {
                words.remove(0);
                parse_oprand(words.first().unwrap())?
            } else {
                Oprand::Immediate(0)
            },
        )
    };
    Ok(Instruction {
        operation,
        operation_args,
        a,
        b,
    })
}

fn add_warnings(instructions: &Vec<Instruction>, warnings: &mut Vec<CompileMessage>) {
    for (line_num, instruction) in instructions.iter().enumerate() {
        let needed_oprands = instruction
            .operation
            .needed_oprands(&instruction.operation_args);
        if let Some(a) = needed_oprands.0 {
            if std::mem::discriminant(&a) != std::mem::discriminant(&instruction.a) {
                warnings.push(CompileMessage {
                    line: line_num,
                    message: format!(
                        "{:?} takes a {} for oprand A, not a {}",
                        instruction.operation,
                        a.get_oprand_name(),
                        &instruction.a.get_oprand_name()
                    ),
                })
            }
        }
        if let Some(b) = needed_oprands.1 {
            if std::mem::discriminant(&b) != std::mem::discriminant(&instruction.b) {
                warnings.push(CompileMessage {
                    line: line_num,
                    message: format!(
                        "{:?} takes a {} for oprand B, not a {}",
                        instruction.operation,
                        b.get_oprand_name(),
                        &instruction.b.get_oprand_name()
                    ),
                })
            }
        }
    }
    if instructions.len() > 32 {
        warnings.push(CompileMessage {
            line: 32,
            message: format!("Too many lines of instruction ({}/32)", instructions.len()),
        })
    }
}
