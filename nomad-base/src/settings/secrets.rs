//! Secrets configuration for agents.
//!
//! This struct is serialized from a JSON file or built drawing from a hosted
//! secrets manager backend. This struct is then used to finish building an
//! agents `Settings` block (see settings/mod.rs) along with a `NomadConfig`.
//!
//! Example JSON File Format
//! {
//!     "rpcs": {
//!         "ethereum": {
//!             "rpcStyle": "ethereum",
//!             "connection": {
//!                 "type": "http",
//!                 "url": ""
//!             }
//!         },
//!         "moonbeam": {
//!             "rpcStyle": "ethereum",
//!             "connection": {
//!                 "type": "http",
//!                 "url": ""
//!             }
//!         },
//!     },
//!     "transactionSigners": {
//!         "ethereum": {
//!             "type": "hexKey"
//!             "key": "",
//!         },
//!         "moonbeam": {
//!             "type": "hexKey"
//!             "key": "",
//!         },
//!     },
//!     "attestationSigner": {
//!         "key": "",
//!         "type": "hexKey"
//!     }
//! }

use crate::{ChainConf, SignerConf};
use color_eyre::Report;
use serde::Deserialize;
use std::collections::HashMap;
use std::{fs::File, io::BufReader, path::Path};

/// Agent secrets block
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AgentSecrets {
    /// RPC endpoints
    pub rpcs: HashMap<String, ChainConf>,
    /// Transaction signers
    pub transaction_signers: HashMap<String, SignerConf>,
    /// Attestation signers
    pub attestation_signer: SignerConf,
}

impl AgentSecrets {
    /// Get JSON file and deserialize into AgentSecrets
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Report> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let secrets = serde_json::from_reader(reader)?;
        Ok(secrets)
    }
}
