use crate::cli::argument::{ConfigGetArgs, ConfigSetArgs};

pub struct CliConfig;

impl CliConfig {
    pub fn new() -> Self {
        Self
    }

    pub fn set(&self, args: ConfigSetArgs) -> anyhow::Result<()> {
        todo!();
    }

    pub fn get(&self, args: ConfigGetArgs) -> anyhow::Result<()> {
        todo!();
        Ok(())
    }

    pub fn show(&self) -> anyhow::Result<()> {
        todo!();
        Ok(())
    }
}