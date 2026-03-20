// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        Pipeline, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

use crate::KeyVaultClient;

impl KeyVaultClient {
    /// Creates a new KeyVaultClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `credential` - An implementation of [`TokenCredential`](azure_core::credentials::TokenCredential) that can provide an
    /// * `subscription_id` - The ID of the target subscription. The value must be an UUID.
    ///   Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Microsoft.KeyVault")]
    pub fn new(
        credential: Arc<dyn TokenCredential>,
        subscription_id: String,
        options: Option<super::KeyVaultClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let cloud = options
            .client_options
            .cloud
            .as_deref()
            .map_or_else(Default::default, Clone::clone);
        let endpoint = super::endpoint(&cloud)?;
        let scope = String::from(super::audience(&cloud)?) + "/.default";
        let auth_policy: Arc<dyn Policy> =
            Arc::new(BearerTokenAuthorizationPolicy::new(credential, vec![scope]));
        Ok(Self {
            endpoint: Url::parse(endpoint)?,
            subscription_id,
            api_version: options.api_version,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.client_options,
                Vec::default(),
                vec![auth_policy],
                None,
            ),
        })
    }
}
