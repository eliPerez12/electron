use parser::*;

mod parser;

#[derive(Debug)]
struct Alu {
    accumalator: u8,
}

#[derive(Debug)]
struct Registers {
    regs: [u8; 8],
}

impl Registers {
    fn write(&mut self, address: u8, data: u8) {
        let reg = self.regs.get_mut(address as usize);
        if let Some(reg) = reg {
            *reg = data
        }
    }

    fn read(&self, address: u8) -> u8 {
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
            },
            registers.read(instruction.b.data())
        );
        self.accumalator = match instruction.operation {
            Operation::ADD => a_data.wrapping_add(b_data),
            Operation::ADDC => a_data.wrapping_add(b_data).wrapping_add(1),
            _ => 0,
        }
    }
}

#[derive(Debug)]
struct Emulator {
    program: Program,
    program_counter: u8,
    fetch_register: Instruction,
    decode_register: Instruction,
    execute_register: Instruction,
    write_back_register: Instruction,
    alu: Alu,
    registers: Registers,
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
            alu: Alu { accumalator: 0 },
            registers: Registers { regs: [0;8] },
        }
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 1;
        if self.program_counter >= 2u8.pow(Self::ROM_ADDRESS_BITS as u32) {
            self.program_counter = 0
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
            Operation::OUT => todo!(),
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

fn main() {
    let program = ProgramLoader::load_program("program.elt");
    let mut emulator = Emulator::new(program);
    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        emulator.clock();
        println!(
            "PC: {:?} | FR: {:?} | DR: {:?} | ER {:?} | WR {:?} | ACCU: {} |  REG1: {} | REG2: {}",
            emulator.program_counter,
            &emulator.fetch_register.operation,
            &emulator.decode_register.operation,
            &emulator.execute_register.operation,
            &emulator.write_back_register.operation,
            &emulator.alu.accumalator,
            &emulator.registers.read(1),
            &emulator.registers.read(2),
        );
    }
}
