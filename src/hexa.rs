// Based on https://github.com/gavr123456789/zed-niva

// use std::path::Path;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct HexaExtension;

impl zed::Extension for HexaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // let hexa_path = Path::new("/home/gavr/.hexa/hexa/bin/hexa");
        // if !hexa_path.exists() {
        //     eprintln!("Error: hexa language server binary not found");
        //     std::process::exit(1);
        // }
        // let x = hexa_path.to_str().unwrap_or("").to_string()

        let path = worktree
            .which("hexals")
            .ok_or_else(|| "hexa must be installed and available on your $PATH".to_string())?;
        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(HexaExtension);
