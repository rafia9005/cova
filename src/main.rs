mod editor;
mod input;
mod mode;
mod rendering;
mod command;

fn main() -> std::io::Result<()> {
    editor::run()
}
