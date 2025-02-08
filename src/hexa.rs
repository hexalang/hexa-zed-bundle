// Based on https://github.com/gavr123456789/zed-niva

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
        let path = worktree
            .which("hexa")
            .ok_or_else(|| "hexa must be installed and available on your $PATH".to_string())?;
        Ok(zed::Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(HexaExtension);
