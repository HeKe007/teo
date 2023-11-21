use std::process::exit;
use std::env::current_dir;
use teo_result::{Error, Result};
use teo_runtime::namespace::Namespace;
use crate::app::ctx::Ctx;
use teo_runtime::utils::find_main_schema_file;
use crate::cli::parse::{parse as cli_parse};
use teo_parser::{parse as schema_parse};
use teo_parser::diagnostics::printer::print_diagnostics;
use teo_runtime::stdlib::load::{load as load_std};
use teo_runtime::schema::load::load_schema::load_schema;
use crate::cli::run::run;
use dotenvy::dotenv;

#[derive(Debug)]
pub struct App { }

impl App {

    pub fn new() -> Result<Self> {
        // load env first
        let _ = dotenv();
        if !Ctx::create() {
            Err(Error::new("cannot create app while there is an existing instance"))?
        }
        let cli = cli_parse(Ctx::get().runtime_version.clone(), Ctx::get().entrance);
        let current_dir = match current_dir() {
            Ok(current_dir) => current_dir,
            Err(e) => Err(Error::new(format!("{}", e)))?,
        };
        let main_schema_file = find_main_schema_file(cli.schema.as_ref().map(AsRef::as_ref), &current_dir)?;
        let (schema, diagnostics) = schema_parse(main_schema_file.as_path().to_str().unwrap(), None, None);
        print_diagnostics(&diagnostics, true);
        if diagnostics.has_errors() {
            exit(1);
        }
        load_std(Ctx::main_namespace_mut());
        Ctx::set_schema(schema);
        Ctx::set_cli(cli);
        Ok(Self { })
    }

    pub fn main_namespace(&self) -> &Namespace {
        Ctx::main_namespace()
    }

    pub fn main_namespace_mut(&self) -> &mut Namespace {
        Ctx::main_namespace_mut()
    }

    pub async fn run(&self) -> Result<()> {
        load_schema(Ctx::main_namespace_mut(), Ctx::schema(), Ctx::cli().command.ignores_loading())?;

        run(Ctx::cli()).await
    }
}