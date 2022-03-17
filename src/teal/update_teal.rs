#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::fs::ReadDir;
    use std::{
        env,
        fs::{self, OpenOptions},
        io::Write,
        path::Path,
        process::Command,
    };

    #[test]
    fn update_teal() -> Result<()> {
        let core_path = Path::new("../../core");

        // Update core's TEAL, to ensure that the copied TEAL is up to date (corresponds to PyTeal)
        update_teal_in_core(core_path)?;

        // Copy the TEAL to this dao (as strings in Rust files)
        let core_teal_templates_dir = fs::read_dir(core_path.join("teal_template"))?;
        let core_teal_dir = fs::read_dir(core_path.join("teal"))?;
        import_teal_from(core_teal_templates_dir)?;
        import_teal_from(core_teal_dir)?;

        println!("TEAL updated");

        Ok(())
    }

    /// Updates TEAL in core
    /// Mote specifically: Calls a core script that compiles core's PyTeal and overwrites core's TEAL files.
    fn update_teal_in_core(core_path: &Path) -> Result<()> {
        // the update teal script uses paths relative to core, so we switch to core's dir
        let initial_dir = env::current_dir()?;
        env::set_current_dir(&core_path)?;

        let core_teal_update_script_path = core_path.join("scripts/update_teal.sh");

        let script_res = Command::new("sh")
            .arg(core_teal_update_script_path)
            .status()?;
        println!("Update core TEAL script res: {:?}", script_res);

        env::set_current_dir(&initial_dir)?;

        Ok(())
    }

    fn import_teal_from(dir: ReadDir) -> Result<()> {
        let wasm_teal_path = Path::new("./src/teal");

        for entry in dir {
            let path = entry?.path();
            let file_stem = path.file_stem().unwrap().to_str().unwrap();

            // Ignore files
            if [
                "always_succeeds",   // this is just for debugging / tests
                "app_capi_clear",    // related to capi asset
                "app_capi_approval", // related to capi asset
                "capi_escrow",       // related to capi asset
            ]
            .contains(&file_stem)
            {
                continue;
            }

            let path_to_write = wasm_teal_path.join(format!("{}.rs", file_stem));

            let teal_to_copy = fs::read(path)?;

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path_to_write)
                .unwrap();

            let mut contents: Vec<u8> = b"pub const SRC: &str = r#\"".to_vec();
            contents.extend(b"\n".to_vec());
            contents.extend(&teal_to_copy);
            contents.extend(b"\n".to_vec());
            contents.extend(b"\"#;".to_vec());
            file.write_all(&contents)?;
        }
        Ok(())
    }
}
