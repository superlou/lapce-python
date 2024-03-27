use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url},
        Request,
    },
    register_plugin, LapcePlugin, PLUGIN_RPC,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    // PLUGIN_RPC.stderr("Initializing python-lapce");

    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(String::from("python")),
        pattern: Some(String::from("**.py")),
        scheme: None,
    }];

    let server_args = vec![];

    let server_path = params
        .initialization_options
        .as_ref()
        .and_then(|options| options.get("serverPath"))
        .and_then(|server_path| server_path.as_str())
        .and_then(|server_path| {
            if !server_path.is_empty() {
                Some(server_path)
            } else {
                None
            }
        });

    if let Some(server_path) = server_path {
        PLUGIN_RPC.start_lsp(
            Url::parse(&format!("urn:{}", server_path))?,
            server_args,
            document_selector,
            params.initialization_options,
        );
        return Ok(());
    }

    let server_path = Url::parse("urn:pylsp")?;

    // PLUGIN_RPC.stderr(&format!("path: {server_path}"));

    PLUGIN_RPC.start_lsp(
        server_path,
        server_args,
        document_selector,
        params.initialization_options,
    );

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                let _ = initialize(params);
            }
            _ => {}
        }
    }
}
