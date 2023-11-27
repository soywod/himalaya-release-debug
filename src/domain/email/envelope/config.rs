use ::serde::{Deserialize, Serialize};

use crate::backend::BackendKind;

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct EnvelopeConfig {
    pub list: Option<EnvelopeListConfig>,
    pub get: Option<EnvelopeGetConfig>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct EnvelopeListConfig {
    pub backend: Option<BackendKind>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct EnvelopeGetConfig {
    pub backend: Option<BackendKind>,
}
