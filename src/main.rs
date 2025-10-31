mod commands;
mod repl;
mod settings;
mod storage;
mod utils;

fn main() {
    let settings = settings::init();
    repl::start_repl(settings);
}
