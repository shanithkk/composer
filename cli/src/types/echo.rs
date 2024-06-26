use super::*;

impl Parser for Composer {
    fn parse(&self, files: &SourceFiles) -> result::Result<()> {
        match self.compile(&format!("{}.{}", ENTRY_FILE, FILE_EXTENSION), files, &mut HashMap::new()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(IOError::Anyhow(err))),
        }
    }

    fn build(
        &self,
        build_directory: &BuildDirectory,
        output_directory: &OutputDirectory,
        quiet: bool,
    ) -> result::Result<()> {
        self.build_directory(&build_directory.path, output_directory.base(), quiet)
            .map_err(|error| Box::new(IOError::Anyhow(error)) as Box<dyn Exception>)?;
        Ok(())
    }
}
