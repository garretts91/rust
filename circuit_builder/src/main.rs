use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Component {
    Resistor(f32),
    Wire,
}

struct Circuit {
    components: HashMap<(usize, usize), Component>,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            components: HashMap::new(),
        }
    }

    fn add_component(&mut self, x: usize, y: usize, component: Component) {
        self.components.insert((x, y), component);
    }

    fn display(&self) {
        for y in 0..10 {
            for x in 0..10 {
                if let Some(component) = self.components.get(&(x, y)) {
                    match component {
                        Component::Resistor(value) => print!(" R{:<3} ", value),
                        Component::Wire => print!(" --- "),
                    }
                } else {
                    print!("     ");
                }
            }
            println!();
        }
    }
}
fn main() {
    let mut circuit = Circuit::new();

    loop {
        println!("1. Add Resistor");
        println!("2. Add Wire");
        println!("3. Display Circuit");
        println!("4. Quit");

        let choice: u32 = read_user_input();

        match choice {
            1 => {
                println!("Enter resistor value:");
                let value: f32 = read_user_input();
                println!("Enter x-coordinate:");
                let x: usize = read_user_input();
                println!("Enter y-coordinate:");
                let y: usize = read_user_input();
                circuit.add_component(x, y, Component::Resistor(value));
            }
            2 => {
                println!("Enter x-coordinate:");
                let x: usize = read_user_input();
                println!("Enter y-coordinate:");
                let y: usize = read_user_input();
                circuit.add_component(x, y, Component::Wire);
            }
            3 => circuit.display(),
            4 => break,
            _ => println!("Invalid Choice!"),
        }
    }
}

fn read_user_input<T>() -> T 
where 
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}