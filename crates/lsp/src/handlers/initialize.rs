use std::sync::Arc;

use super::{semantic_tokens, update_configuration};
use crate::config::InitConfig;
use crate::diagnostics::publish_all_diagnostics;
use crate::environment::Environment;
use crate::world::{Workspace, DEFAULT_WORKSPACE_URL};
use crate::World;
use lsp_async_stub::{rpc::Error, Context, Params};
use lsp_types::{
    CompletionOptions, DeclarationCapability, FoldingRangeProviderCapability,
    HoverProviderCapability, InitializedParams, OneOf, RenameOptions, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensServerCapabilities,
    ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
    WorkDoneProgressOptions, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
};
use lsp_types::{InitializeParams, InitializeResult};

#[tracing::instrument(skip_all)]
pub async fn initialize<E: Environment>(
    context: Context<World<E>>,
    params: Params<InitializeParams>,
) -> Result<InitializeResult, Error> {
    let p = params.required()?;

    if let Some(init_opts) = p.initialization_options {
        match serde_json::from_value::<InitConfig>(init_opts) {
            Ok(c) => context.init_config.store(Arc::new(c)),
            Err(error) => {
                tracing::error!(%error, "invalid initialization options");
            }
        }
    }

    if let Some(workspaces) = p.workspace_folders {
        let mut wss = context.workspaces.write().await;

        for workspace in workspaces {
            wss.entry(workspace.uri.clone())
                .or_insert(Workspace::new(context.env.clone(), workspace.uri));
        }
    }

    Ok(InitializeResult {
        capabilities: ServerCapabilities {
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                ..Default::default()
            }),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            rename_provider: Some(OneOf::Right(RenameOptions {
                prepare_provider: Some(true),
                work_done_progress_options: Default::default(),
            })),
            folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
            references_provider: Some(OneOf::Left(true)),
            declaration_provider: Some(DeclarationCapability::Simple(true)),
            definition_provider: Some(OneOf::Left(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: false.into(),
                    },
                    legend: SemanticTokensLegend {
                        token_types: semantic_tokens::TokenType::LEGEND.into(),
                        token_modifiers: semantic_tokens::TokenModifier::MODIFIERS.into(),
                    },
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                    range: Some(false),
                }),
            ),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(false),
                trigger_characters: Some(vec!["#".into(), "=".into(), ".".into(), ":".into()]),
                ..CompletionOptions::default()
            }),
            ..Default::default()
        },
        server_info: Some(ServerInfo {
            name: "Rhai Language Server".into(),
            version: Some(env!("CARGO_PKG_VERSION").into()),
        }),
    })
}

#[tracing::instrument(skip_all)]
pub async fn initialized<E: Environment>(
    context: Context<World<E>>,
    _params: Params<InitializedParams>,
) {
    update_configuration(context.clone()).await;

    let mut workspaces = context.workspaces.write().await;

    for (ws_url, ws) in workspaces.iter_mut() {
        if ws_url == &*DEFAULT_WORKSPACE_URL {
            continue;
        }

        if let Err(error) = ws.load_rhai_config().await {
            tracing::error!(%error, "failed to load Rhai config");
        }

        ws.load_all_files().await;
    }

    drop(workspaces);
    context
        .clone()
        .all_diagnostics_debouncer
        .spawn(publish_all_diagnostics(context));
}
