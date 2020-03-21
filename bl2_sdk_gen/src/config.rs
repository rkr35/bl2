use std::path::Path;

pub struct Config<'a> {
    pub output_directory: &'a Path,
}

impl<'a> Config<'a> {
    // todo: Read from file
    // todo: Builder
    pub fn new() -> Config<'static> {
        const OUTPUT_DIR: &str = r"C:\Users\Royce\source\repos\bl2\src\sdk";

        Config {
            output_directory: Path::new(OUTPUT_DIR),
        }
    }
}