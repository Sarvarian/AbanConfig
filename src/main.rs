use appinput::AppInput;
use structopt::StructOpt;

mod appinput;
mod command_gen;
mod command_init;
mod command_new;
mod constants;

fn main() {
    let app_in = AppInput::from_args();

    match app_in {
        AppInput::Gen(options) => command_gen::gen(options),
        AppInput::New(options) => command_new::new(options),
        AppInput::Init(options) => command_init::init(options),
    }

    println!("Aban Config Done.");
}
