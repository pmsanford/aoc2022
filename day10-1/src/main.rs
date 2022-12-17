use anyhow::Result;
use util::Input;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl Instruction {
    fn cycle_count(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        if value == "noop" {
            return Instruction::Noop;
        } else {
            let (inst, arg) = value.split_once(' ').unwrap();
            if inst == "addx" {
                let arg: isize = arg.parse().unwrap();
                return Instruction::Addx(arg);
            }
        }

        panic!("Bad instruction");
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
struct Cpu {
    X: isize,
    PC: usize,
    instructions: Vec<Instruction>,
    current_cycle: usize,
    instruction_cycles: usize,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            X: 1,
            PC: 0,
            current_cycle: 1,
            instructions,
            instruction_cycles: 0,
        }
    }

    fn cycle(&mut self) {
        if self.instruction_cycles == 0 {
            self.instruction_cycles = self.instructions[self.PC].cycle_count();
        }
        self.instruction_cycles -= 1;
        if self.instruction_cycles == 0 {
            match self.instructions[self.PC] {
                Instruction::Noop => {}
                Instruction::Addx(v) => {
                    self.X += v;
                }
            }
            self.PC += 1;
        }
        self.current_cycle += 1;
    }
}

fn main() -> Result<()> {
    let input = Input::new()?
        .into_lines()?
        .into_iter()
        .map(Instruction::from)
        .collect::<Vec<_>>();

    let mut cpu = Cpu::new(input);

    let mut total = 0;

    loop {
        if cpu.current_cycle == 20 || (cpu.current_cycle as isize - 20) % 40 == 0 {
            let signal_strength = cpu.current_cycle as isize * cpu.X;
            println!(
                "Cycle {}: \n\tX: {}\n\tSignal strength: {}",
                cpu.current_cycle, cpu.X, signal_strength
            );
            total += signal_strength;
        }

        cpu.cycle();

        if cpu.PC >= cpu.instructions.len() {
            break;
        }
    }

    println!();
    println!("Total: {total}");

    Ok(())
}
