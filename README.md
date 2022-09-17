# lapce-python

The python language server pyls must be available on the path.
Install the python language server by `pip install python-lsp-server`.

Additional functionality like type checking, refactoring, and formatting can be provided by installing dependencies as described in the [README for python-lsp-server](https://github.com/python-lsp/python-lsp-server).

## Build

```
rustup target add wasm32-wasi
cargo build
```

## Develop

On OSX,

```
pip install pipx
pipx install python-lsp-server
# adds ~/.local/bin/pylsp
```

```
cd ~/Library/Application Support/dev.lapce.Lapce/plugins # or dev.lapce.Lapce-Stable
# exact files to link may vary with lapce releases
mkdir lapce-python; cd lapce-python
ln -s ~/prog/lapce-python/target/wasm32-wasi/debug/lapce-python.wasm .
```

Run lapce from the terminal to see error messages.

```
RUST_BACKTRACE=1 /Applications/Lapce.app/Contents/MacOS/lapce
```
