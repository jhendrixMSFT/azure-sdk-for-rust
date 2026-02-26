// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

#[allow(unused_imports)]
mod generated;

mod logging;

use crate::models::SentMessage;
use azure_core::time::OffsetDateTime;
use serde::Deserialize;

impl<'de> Deserialize<'de> for SentMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "QueueMessagesList")]
        struct ListOfSentMessage {
            #[serde(rename = "QueueMessage", skip_serializing_if = "Option::is_none")]
            pub items: Option<Vec<SentMessageInner>>,
        }

        #[derive(Deserialize)]
        struct SentMessageInner {
            #[serde(
                default,
                rename = "ExpirationTime",
                skip_serializing_if = "Option::is_none",
                with = "azure_core::time::rfc7231::option"
            )]
            pub expiration_time: Option<OffsetDateTime>,

            #[serde(
                default,
                rename = "InsertionTime",
                skip_serializing_if = "Option::is_none",
                with = "azure_core::time::rfc7231::option"
            )]
            pub insertion_time: Option<OffsetDateTime>,

            #[serde(rename = "MessageId", skip_serializing_if = "Option::is_none")]
            pub message_id: Option<String>,

            #[serde(rename = "PopReceipt", skip_serializing_if = "Option::is_none")]
            pub pop_receipt: Option<String>,

            #[serde(
                default,
                rename = "TimeNextVisible",
                skip_serializing_if = "Option::is_none",
                with = "azure_core::time::rfc7231::option"
            )]
            pub time_next_visible: Option<OffsetDateTime>,
        }

        let list = ListOfSentMessage::deserialize(deserializer)?;
        let message = list
            .items
            .unwrap_or_default()
            .into_iter()
            .next()
            .ok_or_else(|| serde::de::Error::custom("No messages found in the response."))?;

        Ok(Self {
            expiration_time: message.expiration_time,
            insertion_time: message.insertion_time,
            message_id: message.message_id,
            pop_receipt: message.pop_receipt,
            time_next_visible: message.time_next_visible,
        })
    }
}

/// Data models and types used by the Azure Storage Queue service.
///
/// This module contains all the request and response models, enums, and other data types
/// used when interacting with Azure Storage Queues, including queue messages, metadata,
/// and service properties.
pub mod models {
    pub use crate::generated::models::*;
}

/// Client implementations for interacting with Azure Storage Queue service.
///
/// This module provides high-level client APIs for managing queues and queue messages,
/// including operations like creating queues, sending/receiving messages, and managing
/// queue metadata.
pub mod clients;

pub use clients::{QueueClient, QueueClientOptions, QueueServiceClient, QueueServiceClientOptions};
