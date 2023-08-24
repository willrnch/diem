// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    get_version_path_with_base, Client, DEFAULT_VERSION_PATH_BASE, X_DIEM_SDK_HEADER_VALUE,
};
use anyhow::Result;
use diem_api_types::X_DIEM_CLIENT;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder,
};
use std::{str::FromStr, time::Duration};
use url::Url;

pub enum DiemBaseUrl {
    Mainnet,
    Devnet,
    Testnet,
    Custom(Url),
}

impl DiemBaseUrl {
    pub fn to_url(&self) -> Url {
        match self {
            DiemBaseUrl::Mainnet => {
                Url::from_str("https://fullnode.mainnet.diemlabs.com").unwrap()
            },
            DiemBaseUrl::Devnet => Url::from_str("https://fullnode.devnet.diemlabs.com").unwrap(),
            DiemBaseUrl::Testnet => {
                Url::from_str("https://fullnode.testnet.diemlabs.com").unwrap()
            },
            DiemBaseUrl::Custom(url) => url.to_owned(),
        }
    }
}

pub struct ClientBuilder {
    reqwest_builder: ReqwestClientBuilder,
    version_path_base: String,
    base_url: Url,
    timeout: Duration,
    headers: HeaderMap,
}

impl ClientBuilder {
    pub fn new(diem_base_url: DiemBaseUrl) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            X_DIEM_CLIENT,
            HeaderValue::from_static(X_DIEM_SDK_HEADER_VALUE),
        );

        Self {
            reqwest_builder: ReqwestClient::builder(),
            base_url: diem_base_url.to_url(),
            version_path_base: DEFAULT_VERSION_PATH_BASE.to_string(),
            timeout: Duration::from_secs(10), // Default to 10 seconds
            headers,
        }
    }

    pub fn base_url(mut self, base_url: Url) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn header(mut self, header_key: &str, header_val: &str) -> Result<Self> {
        self.headers.insert(
            HeaderName::from_str(header_key)?,
            HeaderValue::from_str(header_val)?,
        );
        Ok(self)
    }

    pub fn version_path_base(mut self, version_path_base: String) -> Self {
        self.version_path_base = version_path_base;
        self
    }

    pub fn build(self) -> Client {
        let version_path_base = get_version_path_with_base(self.base_url.clone());

        Client {
            inner: self
                .reqwest_builder
                .default_headers(self.headers)
                .timeout(self.timeout)
                .cookie_store(true)
                .build()
                .unwrap(),
            base_url: self.base_url,
            version_path_base,
        }
    }
}
