use lapce_plugin::{register_plugin, start_lsp, LapcePlugin};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    arch: String,
    os: String,
    configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    language_id: String,
    options: Option<Value>,
}

register_plugin!(State);

impl LapcePlugin for State {
    fn initialize(&mut self, info: serde_json::Value) {
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();
        let _arch = match info.arch.as_str() {
            "x86_64" => "x86_64",
            "aarch64" => "aarch64",
            _ => return,
        };
        let _os = match info.os.as_str() {
            "linux" => "unknown-linux-gnu",
            "macos" => "apple-darwin",
            "windows" => "pc-windows-msvc",
            _ => return,
        };

        // lapce-rust does this; is there any handler?
        // let lock_file = PathBuf::from("download.lock");
        // send_notification(
        //     "python",
        //     &json!({
        //         "path": &lock_file,
        //     }),
        // );

        let pylsp_path = match env::var("PYLSP_PATH") {
            Ok(var) => var,
            Err(error) => panic!("Couldn't get PYLSP_PATH: {error}"),
        };
        dbg!(&pylsp_path);
        // two copies of us are started
        serde_json::to_writer_pretty(std::io::stderr(), &info).unwrap();

        start_lsp(&pylsp_path, "python", info.configuration.options);
    }
}
