# lapce-python

The python language server pyls must be available on the path.
Install the python language server by `pip install 'python-language-server[all]'`.

If you are on Windows, try changing the `env_command` in `plugin.toml` to:

```
env_command = "echo \"PYLS_PATH=$(where pyls)\""
```

## Build

```
rustup target add wasm32-wasi
cargo build
```

## Develop

On OSX,

```
pip install pipx
pipx install pylsp
# adds ~/.local/bin/pylsp
```

```
cd ~/.lapce/plugins
mkdir lapce-python; cd lapce-python
ln -s ~/prog/lapce-python/plugin.toml .
ln -s ~/prog/lapce-python/target/wasm32-wasi/debug/lapce-python.wasm .
```

Run lapce from the terminal to see error messages. `lapce-python` should serialize its configuration to stderr.

```
RUST_BACKTRACE=1 /Applications/Lapce.app/Contents/MacOS/lapce
```

Unfortunately broken language servers seem to prevent lapce from being able to save.