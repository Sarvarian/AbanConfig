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
    about = "Generate code and other type of files. (See help)"
)]
pub enum GenOptions {
    Src(GenSrcOptions),
    CMake(GenCMakeOptions),
    PS1(GenPS1Options),
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

// ----- Gen Subcommands -----

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Generate Source Files",
    about = "Generate Source File based on modules toml files."
)]
pub struct GenSrcOptions {}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Generate CMake File",
    about = "Generate CMake file based on aban-templates/CMakeList.txt file."
)]
pub struct GenCMakeOptions {}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Generate Powershell Script",
    about = "Generate Powershell script to run cmake and build."
)]
pub struct GenPS1Options {}

// g
// g
// G
// G
