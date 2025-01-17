// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developersopers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr_ffi::helper::unwrap_or_clone_arc;
use nostr_sdk::database::DynNostrDatabase;
use uniffi::Object;

use super::{Client, ClientSdk, ClientSigner, Options};
use crate::database::NostrDatabase;

#[derive(Clone, Object)]
pub struct ClientBuilder {
    inner: nostr_sdk::ClientBuilder,
}

impl From<nostr_sdk::ClientBuilder> for ClientBuilder {
    fn from(inner: nostr_sdk::ClientBuilder) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl ClientBuilder {
    /// New client builder
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: nostr_sdk::ClientBuilder::new(),
        }
    }

    pub fn signer(self: Arc<Self>, signer: Arc<ClientSigner>) -> Self {
        let signer: nostr_sdk::ClientSigner = signer.as_ref().deref().clone();
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.signer(signer);
        builder
    }

    pub fn database(self: Arc<Self>, database: Arc<NostrDatabase>) -> Self {
        let database: Arc<DynNostrDatabase> = database.as_ref().into();
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.database(database);
        builder
    }

    /// Set opts
    pub fn opts(self: Arc<Self>, opts: Arc<Options>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.opts(opts.as_ref().deref().clone());
        builder
    }

    /// Build [`Client`]
    pub fn build(&self) -> Arc<Client> {
        let mut inner = self.inner.clone();
        inner.opts = inner.opts.shutdown_on_drop(true);
        Arc::new(ClientSdk::from_builder(inner).into())
    }
}
