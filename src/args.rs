use std::str::FromStr;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(help = "The file to compile")]
    pub file: String,

    #[clap(long, help = "Print out the input file annotated with inferred lifetimes of heap allocations")]
    pub show_lifetimes: bool,

    #[clap(long, help = "Lex the file and output the resulting list of tokens")]
    pub lex: bool,

    #[clap(long, help = "Parse the file and output the resulting Ast")]
    pub parse: bool,

    #[clap(long, help = "Check the file for errors without compiling")]
    pub check: bool,

    #[clap(long, help = "Build the resulting binary without running it afterward")]
    pub build: bool,

    #[clap(
        long,
        help = "Specify the backend to use ('llvm' or 'cranelift'). Note that cranelift is only for debug builds. Ante will use cranelift by default for debug builds and llvm by default for optimized builds, unless overridden by this flag"
    )]
    pub backend: Option<Backend>,

    #[clap(
        short = 'O',
        default_value = "0",
        validator(validate_opt_argument),
        help = "Sets the current optimization level from 0 (no optimization) to 3 (aggressive optimization). Set to s or z to optimize for size."
    )]
    pub opt_level: char,

    #[clap(long, help = "Use plaintext and an indicator line instead of color for pointing out error locations")]
    pub no_color: bool,

    #[clap(long, help = "Print out the LLVM-IR or Cranelift IR of the compiled program")]
    pub show_ir: bool,

    #[clap(long, help = "Print out the HIR, Ante's post-monomorphisation IR")]
    pub show_hir: bool,

    #[clap(long, help = "Delete the resulting binary after compiling")]
    pub delete_binary: bool,

    #[clap(long, help = "Print out the time each compiler pass takes for the given program")]
    pub show_time: bool,

    #[clap(long, help = "Print out the type of each definition")]
    pub show_types: bool,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Backend {
    Cranelift,
    Llvm,
}

impl FromStr for Backend {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cranelift" => Ok(Backend::Cranelift),
            "llvm" => Ok(Backend::Llvm),
            _ => Err("Unknown backend. Valid backends are 'llvm' and 'cranelift'"),
        }
    }
}

fn validate_opt_argument(arg: &str) -> Result<(), &'static str> {
    match arg {
        "0" | "1" | "2" | "3" | "s" | "z" => Ok(()),
        _ => Err("Argument to -O must be one of: 0, 1, 2, 3, s, or z"),
    }
}
