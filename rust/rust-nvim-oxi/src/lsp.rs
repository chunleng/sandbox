use nvim_oxi::{
    Dictionary, Object,
    api::{self, opts::CreateAutocmdOpts, types::AutocmdCallbackArgs},
    conversion::FromObject,
};

const LSP_BIN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/debug/sample-lsp");

// Wires the sample-lsp client into Neovim: registers the config, enables
// snippet expansion on completion-accept via LspAttach, then enables the
// client. Must run before any markdown buffer attaches for the LspAttach
// autocmd to catch already-open buffers.
pub fn setup() {
    let config = format!(
        "vim.lsp.config('sample-lsp', {{ cmd = {{ '{LSP_BIN}' }}, filetypes = {{ 'markdown' }} }})"
    );
    api::command(&format!("lua {config}")).unwrap();
    // Snippet expansion on completion-accept is only wired up by
    // vim.lsp.completion.enable; omnifunc alone inserts raw text.
    // Registered before vim.lsp.enable so LspAttach for already-open
    // markdown buffers is also caught.
    let opts = CreateAutocmdOpts::builder()
        .desc("Enable native snippet expansion for sample-lsp completions")
        .callback(|args: AutocmdCallbackArgs| {
            let client_id = Dictionary::from_object(args.data)
                .ok()
                .and_then(|d: Dictionary| d.get("client_id").cloned())
                .and_then(|id: Object| i64::try_from(id).ok())
                .unwrap_or_default();
            let cmd = format!(
                "vim.lsp.completion.enable(true, {client_id}, {})",
                args.buffer.handle()
            );
            api::command(&format!("lua {cmd}"))?;
            Ok::<bool, nvim_oxi::Error>(false)
        })
        .build();
    api::create_autocmd(["LspAttach"], &opts).unwrap();
    api::command("lua vim.lsp.enable('sample-lsp')").unwrap();
}
