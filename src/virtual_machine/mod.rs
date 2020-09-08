use std::ops::Add;

pub fn execute<I, C, L>(cpu: C, list: L) -> C
where
    L: Iterator<Item = I>,
    C: CPU<ISA = I>,
{
    list.fold(cpu, |cpu, instruction| cpu.execute(instruction))
}

// TODO :: Why does execute needs to be sized?
pub trait CPU {
    type ISA;
    fn execute(self, instruction: Self::ISA) -> Self;
    fn executes<L>(self, instructions: L) -> Self
    where
        L: Iterator<Item = Self::ISA>,
        Self: Sized,
    {
        instructions.fold(self, |cpu, instruction| cpu.execute(instruction))
    }
}

#[derive(Copy, Clone)]
pub enum BasicRegister {
    A,
    B,
    C,
}

impl Register for BasicRegister {}

impl From<BasicRegister> for usize {
    fn from(from: BasicRegister) -> Self {
        match from {
            BasicRegister::A => 0,
            BasicRegister::B => 1,
            BasicRegister::C => 2,
        }
    }
}

pub trait Register {}

pub enum BasicIsa<T, R>
where
    R: Register,
{
    /// Adds A and B placing it in C.
    ADD(R, R, R),
    /// Copies A into B.
    COPY(R, R),
    /// Sets A to B.
    SET(T, R),
}

#[derive(Eq, PartialEq, Debug)]
pub struct BasicCPU<T> {
    registers: [T; 3],
}

impl<T> Default for BasicCPU<T>
where
    T: Default,
{
    fn default() -> Self {
        BasicCPU {
            registers: [T::default(), T::default(), T::default()],
        }
    }
}

impl<T> CPU for BasicCPU<T>
where
    T: Add<Output = T> + Copy,
{
    type ISA = BasicIsa<T, BasicRegister>;

    fn execute(self, isa: Self::ISA) -> Self {
        match isa {
            BasicIsa::ADD(a, b, c) => {
                let mut registers = self.registers;
                registers[usize::from(c)] = registers[usize::from(a)] + registers[usize::from(b)];
                Self { registers }
            }
            BasicIsa::COPY(a, b) => {
                let mut registers = self.registers;
                registers[usize::from(b)] = self.registers[usize::from(a)];
                Self { registers }
            }
            BasicIsa::SET(a, b) => {
                let mut registers = self.registers;
                registers[usize::from(b)] = a;
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
    {
        let cpu = BasicCPU {
            registers: [1, 2, 0],
        };
        let instruction = BasicIsa::ADD(BasicRegister::A, BasicRegister::B, BasicRegister::C);
        assert_eq!(
            BasicCPU {
                registers: [1, 2, 3]
            },
            cpu.execute(instruction)
        );
    }
    Ok(())
}

#[test]
pub fn basic_virtual_machine_consuming_instructions() -> Result<(), Box<dyn std::error::Error>> {
    let instructions = vec![
        BasicIsa::ADD(BasicRegister::A, BasicRegister::C, BasicRegister::C),
        BasicIsa::ADD(BasicRegister::A, BasicRegister::C, BasicRegister::C),
        BasicIsa::ADD(BasicRegister::A, BasicRegister::C, BasicRegister::C),
        BasicIsa::ADD(BasicRegister::A, BasicRegister::C, BasicRegister::C),
        BasicIsa::ADD(BasicRegister::A, BasicRegister::C, BasicRegister::C),
        BasicIsa::COPY(BasicRegister::C, BasicRegister::B),
    ];
    let cpu = BasicCPU {
        registers: [1, 0, 0],
    };
    assert_eq!(
        BasicCPU {
            registers: [1, 5, 5]
        },
        cpu.executes(instructions.into_iter())
    );
    Ok(())
}
