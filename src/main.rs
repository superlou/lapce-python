use anyhow::{anyhow, Result};
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

macro_rules! ok {
    ( $x:expr ) => {
        match ($x) {
            Ok(v) => v,
            Err(e) => return Err(anyhow!(e)),
        }
    };
}

fn initialize(params: InitializeParams) -> Result<()> {
    // PLUGIN_RPC.stderr("Initializing python-lapce");

    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(String::from("python")),
        pattern: Some(String::from("**.py")),
        scheme: None,
    }];

    let server_args = vec![];

    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(volt) = options.get("volt") {
            if let Some(server_path) = volt.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let server_uri = ok!(Url::parse(&format!("urn:{}", server_path)));
                        PLUGIN_RPC.start_lsp(
                            server_uri,
                            server_args,
                            document_selector,
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }
        }
    }

    let server_path = Url::parse("urn:pylspz")?;

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
