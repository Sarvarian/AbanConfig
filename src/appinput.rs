use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Aban Project Management Tools",
    about = "This app provide some tools to manage your Aban project."
)]
pub enum AppInput {
    Gen(GenOptions),
    New(NewOptions),
    Init(InitOptions),
}

// ----- Subcommands -----

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Code Generator",
    about = "Generate CMakeLists.txt and other source files."
)]
pub struct GenOptions {
    pub start_path: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "New Aban Project", about = "Generate new Aban project.")]
pub struct NewOptions {
    pub path: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Init Aban Project Here",
    about = "Initialize current directory for an aban project."
)]
pub struct InitOptions {}

// g
// g
// G
// G
