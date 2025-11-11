use zed_extension_api as zed;

/// Main extension struct implementing the Extension trait
pub struct DemoExtension {
    // Extension state can be stored here
}

impl zed::Extension for DemoExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_name: &str,
        _worktree: &zed::Worktree,
    ) -> Option<zed::Command> {
        // Return a language server command if needed
        // Example:
        // if language_name == "python" {
        //     Some(zed::Command {
        //         command: "pylsp".to_string(),
        //         args: vec![],
        //         env: Default::default(),
        //     })
        // } else {
        //     None
        // }
        None
    }

    fn indexing_settings(&mut self, _worktree: &zed::Worktree) -> Option<zed::IndexingSettings> {
        // Configure indexing behavior for specific file types
        None
    }
}

// Register the extension with Zed
zed::register_extension!(DemoExtension);