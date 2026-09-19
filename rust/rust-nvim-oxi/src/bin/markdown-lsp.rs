use std::collections::HashMap;

use lsp_server::{Connection, Message, Notification, Request, Response};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, CompletionResponse,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InsertTextFormat,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};

fn main() {
    let (connection, io_threads) = Connection::stdio();

    let capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        completion_provider: Some(CompletionOptions {
            ..Default::default()
        }),
        ..Default::default()
    };
    let _: InitializeParams = serde_json::from_value(
        connection
            .initialize(serde_json::to_value(capabilities).unwrap())
            .unwrap(),
    )
    .unwrap();

    main_loop(connection);
    io_threads.join().unwrap();
}

fn main_loop(connection: Connection) {
    let mut docs = HashMap::new();
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req).unwrap() {
                    return;
                }
                handle_request(&connection, req, &docs);
            }
            Message::Notification(note) => handle_notification(note, &mut docs),
            Message::Response(_) => {}
        }
    }
}

fn handle_notification(note: Notification, docs: &mut HashMap<String, String>) {
    match note.method.as_str() {
        "textDocument/didOpen" => {
            let params: DidOpenTextDocumentParams = serde_json::from_value(note.params).unwrap();
            docs.insert(
                params.text_document.uri.to_string(),
                params.text_document.text,
            );
        }
        "textDocument/didChange" => {
            let params: DidChangeTextDocumentParams = serde_json::from_value(note.params).unwrap();
            // Full sync: the last change carries the entire document text.
            if let Some(change) = params.content_changes.into_iter().last() {
                docs.insert(params.text_document.uri.to_string(), change.text);
            }
        }
        _ => {}
    }
}

fn handle_request(connection: &Connection, req: Request, docs: &HashMap<String, String>) {
    match req.method.as_str() {
        "textDocument/completion" => {
            let params: CompletionParams = serde_json::from_value(req.params).unwrap();
            let items = if at_line_start(&params, docs) {
                snippets()
            } else {
                vec![]
            };
            let resp = Response::new_ok(req.id.clone(), Some(CompletionResponse::Array(items)));
            connection.sender.send(Message::Response(resp)).unwrap();
        }
        _ => {
            let resp = Response::new_err(
                req.id.clone(),
                lsp_server::ErrorCode::MethodNotFound as i32,
                format!("method not found: {}", req.method),
            );
            connection.sender.send(Message::Response(resp)).unwrap();
        }
    }
}

// The word being completed starts at column 0 when the text before the cursor
// on that line contains no whitespace or other separators.
fn at_line_start(params: &CompletionParams, docs: &HashMap<String, String>) -> bool {
    let pos = &params.text_document_position;
    let Some(text) = docs.get(&pos.text_document.uri.to_string()) else {
        return false;
    };
    let Some(line) = text.lines().nth(pos.position.line as usize) else {
        return false;
    };
    // LSP positions are UTF-16 code units; walk chars instead of slicing bytes
    // so multibyte characters cannot panic on a non-char boundary.
    let mut consumed = 0u32;
    for c in line.chars() {
        if consumed >= pos.position.character {
            break;
        }
        consumed += c.len_utf16() as u32;
        if !(c.is_alphanumeric() || c == '_') {
            return false;
        }
    }
    true
}

// Extensible list of markdown snippets; only `title` for now.
fn snippets() -> Vec<CompletionItem> {
    vec![CompletionItem {
        label: "title".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        insert_text: Some("# ${1:Title}".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        detail: Some("# Title".to_string()),
        ..Default::default()
    }]
}
