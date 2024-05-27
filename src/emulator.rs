use crate::parser::*;

#[derive(Debug)]
pub struct Alu {
    pub accumalator: u8,
    pub flags: AluFlags,
}

#[derive(Debug)]
pub struct AluFlags {
    pub equals: bool,
    pub greater_than: bool,
    pub less_than: bool,
    pub over_flow: bool,
}

#[derive(Debug)]
pub struct Registers {
    regs: [u8; 8],
}

impl Registers {
    pub fn write(&mut self, address: u8, data: u8) {
        let reg = self.regs.get_mut(address as usize);
        if let Some(reg) = reg {
            *reg = data
        }
    }

    pub fn read(&self, address: u8) -> u8 {
        if address == 0 {
            0
        } else {
            *self.regs.get(address as usize).unwrap_or(&0)
        }
    }
}

impl Alu {
    fn execute(&mut self, registers: &Registers, instruction: &Instruction) {
        let (a_data, b_data) = (
            match instruction.operation_args {
                OperationArgs::None => registers.read(instruction.a.data()),
                OperationArgs::S => registers.read(instruction.a.data()),
                OperationArgs::U => self.accumalator,
                OperationArgs::X => self.accumalator,
            } as u16,
            registers.read(instruction.b.data()) as u16,
        );
        let mut result = match instruction.operation {
            Operation::ADD => a_data + b_data,
            Operation::ADDC => a_data + b_data + 1,
            _ => 0,
        };
        self.flags = AluFlags {
            equals: a_data == b_data,
            greater_than: a_data > b_data,
            less_than: a_data < b_data,
            over_flow: result > 255,
        };
        if result > 255 {
            result -= 255;
        }
        self.accumalator = result as u8;
    }
}

#[derive(Debug)]
pub struct Ports {
    pub out: [u8; 8],
    pub input: [u8; 8],
}

impl Ports {
    pub fn write_out(&mut self, address: u8, data: u8) {
        let port = self.out.get_mut(address as usize);
        if let Some(port) = port {
            *port = data
        }
    }

    pub fn _read_in(&self, address: u8) -> u8 {
        *self.input.get(address as usize).unwrap_or(&0)
    }
}

#[derive(Debug)]
pub struct Emulator {
    program: Program,
    pub program_counter: u8,
    pub fetch_register: Instruction,
    pub decode_register: Instruction,
    pub execute_register: Instruction,
    pub write_back_register: Instruction,
    pub alu: Alu,
    pub registers: Registers,
    pub ports: Ports,
}

impl Emulator {
    const ROM_ADDRESS_BITS: u8 = 5;
    pub fn new(program: Program) -> Self {
        Self {
            program,
            program_counter: 0,
            fetch_register: Instruction::none(),
            decode_register: Instruction::none(),
            execute_register: Instruction::none(),
            write_back_register: Instruction::none(),
            alu: Alu {
                accumalator: 0,
                flags: AluFlags {
                    equals: false,
                    greater_than: false,
                    less_than: false,
                    over_flow: false,
                },
            },
            registers: Registers { regs: [0; 8] },
            ports: Ports {
                out: [0; 8],
                input: [0; 8],
            },
        }
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 1;
        if self.program_counter >= 2u8.pow(Self::ROM_ADDRESS_BITS as u32) {
            self.program_counter = 0
        }
    }

    fn check_for_branch(&mut self) {
        match self.execute_register.operation {
            Operation::JMP => self.program_counter = self.execute_register.a.data(),
            _ => (),
        }
    }

    fn fetch(&mut self) {
        self.fetch_register = self.program.instructions[self.program_counter as usize].clone();
    }

    fn decode(&mut self) {
        self.decode_register = self.fetch_register.clone();
    }

    fn execute(&mut self) {
        self.execute_register = self.decode_register.clone();
        //self.program_counter = self.execute_register.a.data();
        self.check_for_branch();
        self.alu.execute(&self.registers, &self.execute_register);
    }

    fn write_back(&mut self) {
        self.write_back_register = self.execute_register.clone();
        let (a, b) = (
            self.write_back_register.a.data(),
            self.write_back_register.b.data(),
        );
        match self.write_back_register.operation {
            Operation::NOOP => (),
            Operation::IMM => self.registers.write(a, b),
            Operation::MOV => self.registers.write(a, self.registers.read(b)),
            Operation::ADD => match self.write_back_register.operation_args {
                OperationArgs::None => (),
                OperationArgs::S => self.registers.write(a, self.alu.accumalator),
                OperationArgs::U => self.registers.write(a, self.alu.accumalator),
                OperationArgs::X => (),
            },
            Operation::ADDC => match self.write_back_register.operation_args {
                OperationArgs::None => (),
                OperationArgs::S => self.registers.write(a, self.alu.accumalator),
                OperationArgs::U => self.registers.write(a, self.alu.accumalator),
                OperationArgs::X => (),
            },
            Operation::OUT => self.ports.write_out(a, self.registers.read(b)),
            Operation::JMP => (),
        }
    }

    pub fn clock(&mut self) {
        self.write_back();
        self.execute();
        self.decode();
        self.fetch();
        self.increment_program_counter();
    }
}
