use program_loader::ProgramLoader;

mod program_loader;

fn main() {
    let program = ProgramLoader::load_program("program.elt");
    println!("Compiling...")
}
