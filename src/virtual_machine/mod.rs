use crate::paper::fold;

pub fn execute<I, C, L>(cpu: C, list: L) -> C
where
    L: Iterator<Item = I>,
    C: CPU<ISA = I>,
{
    fold(|instruction, cpu| cpu.execute(instruction), cpu, list)
}

pub trait CPU {
    type ISA;
    fn execute(self, isa: Self::ISA) -> Self;
}

#[derive(Copy, Clone)]
pub enum BasicRegister {
    A,
    B,
    C,
}

impl From<BasicRegister> for usize {
    fn from(from: BasicRegister) -> Self {
        match from {
            BasicRegister::A => 0,
            BasicRegister::B => 1,
            BasicRegister::C => 2,
        }
    }
}

pub enum BasicIsa<Register> {
    // (InputA, InputB, Output)
    ADD(Register, Register, Register),
}

#[derive(Eq, PartialEq, Debug)]
pub struct BasicCPU {
    registers: [i32; 3],
}

impl Default for BasicCPU {
    fn default() -> Self {
        BasicCPU {
            registers: [0, 0, 0],
        }
    }
}

impl CPU for BasicCPU {
    type ISA = BasicIsa<BasicRegister>;

    fn execute(self, isa: Self::ISA) -> Self {
        match isa {
            BasicIsa::ADD(a, b, c) => {
                let mut registers = self.registers;
                registers[usize::from(c)] = registers[usize::from(a)] + registers[usize::from(b)];
                Self { registers }
            }
        }
    }
}

#[test]
pub fn basic_virtual_machine_test() -> Result<(), Box<dyn std::error::Error>> {
    {
        let cpu = BasicCPU {
            registers: [1, 2, 0],
        };
        let instruction = BasicIsa::ADD(BasicRegister::A, BasicRegister::A, BasicRegister::C);
        assert_eq!(
            BasicCPU {
                registers: [1, 2, 2]
            },
            cpu.execute(instruction)
        );
    }
    Ok(())
}
