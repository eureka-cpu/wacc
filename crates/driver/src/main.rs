use std::{fs, path, process};

use anyhow::Result;
use clap::{ArgGroup, Parser};
use wacc_lexer::{c_token::CToken, Lexer};

#[derive(Parser)]
#[command(
    about = "The compiler driver for Writing a C Compiler.",
    group = ArgGroup::new("mode")
        .args(&["lex", "parse", "codegen"])
        .required(true)
)]
pub struct Command {
    /// The path to a C source file
    c_source_file: String,

    #[arg(help = "lex only, then stop", long)]
    lex: bool,

    #[arg(help = "lex, parse, then stop", long)]
    parse: bool,

    #[arg(help = "lex, parse, generate assembly, then stop", long)]
    codegen: bool,
}

struct WaccCommand;
impl WaccCommand {
    /// Compile the preprocessed source file and output an assembly file with a .s extension.
    fn compile(preprocessed_file: &str, lex: bool, _parse: bool, _codegen: bool) -> Result<String> {
        if lex {
            let source_str = fs::read_to_string(preprocessed_file)?;
            Lexer::lex::<CToken>(&source_str, String::lex_c);
            process::exit(0);
        }

        let (assembly_file, _ext) = preprocessed_file
            .rsplit_once('.')
            .expect("expected a valid filename");
        let mut assembly_file = String::from(assembly_file);
        assembly_file.push_str(".s");

        // TODO: replace this once compiler is written
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&assembly_file)?;

        fs::remove_file(preprocessed_file)
            .map_err(|e| anyhow::anyhow!("couldn't remove preprocessed file: {e:?}"))?;

        Ok(assembly_file)
    }
}

struct GccCommand(process::Command);
impl GccCommand {
    fn new() -> Self {
        Self(process::Command::new("gcc"))
    }

    /// This command preprocesses INPUT_FILE and then writes the result to PREPROCESSED_FILE.
    /// By convention, PREPROCESSED_FILE should have a .i file extension.
    fn preprocess(input_file: &str) -> Result<String> {
        let (preprocessed_file, _ext) = input_file
            .rsplit_once('.')
            .expect("expected a valid filename");
        let mut preprocessed_file = String::from(preprocessed_file);
        preprocessed_file.push_str(".i");

        if !GccCommand::new()
            .0
            .args(["-E", "-P", input_file, "-o", &preprocessed_file])
            .output()?
            .status
            .success()
        {
            eprintln!("failed to produce preprocessed file");
            process::exit(1);
        }

        Ok(preprocessed_file)
    }

    /// Assemble and link the assembly file to produce an executable.
    fn assemble(assembly_file: &str) -> Result<()> {
        let (output_file, _ext) = assembly_file
            .rsplit_once('.')
            .expect("expected a valid filename");

        if !GccCommand::new()
            .0
            .args([assembly_file, "-o", output_file])
            .output()?
            .status
            .success()
        {
            eprintln!("failed to assemble and link executable file");
            process::exit(1);
        }

        fs::remove_file(assembly_file)
            .map_err(|e| anyhow::anyhow!("couldn't remove assembly file: {e:?}"))?;

        process::exit(0);
    }
}

fn main() -> Result<()> {
    let Command {
        c_source_file,
        lex,
        parse,
        codegen,
    } = Command::parse();

    if !path::Path::new(&c_source_file).exists() {
        eprintln!("file not found: {c_source_file}");
        process::exit(1);
    }

    GccCommand::assemble(&WaccCommand::compile(
        &GccCommand::preprocess(&c_source_file)?,
        lex,
        parse,
        codegen,
    )?)
}

#[test]
fn test() {}
