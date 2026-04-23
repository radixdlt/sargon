use identified_vec_of::decl_identified_vec_of;
use std::fmt;

use crate::prelude::*;
use core_misc::parse_url;

decl_identified_vec_of!(
    /// An ordered collection of unique [`RelayService`]s.
    /// The identifier of a relay service is its URL.
    RelayService
);

#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display)]
#[display("{}", self.0)]
pub struct RelayServiceUrl(String);

impl RelayServiceUrl {
    pub fn new(url: impl Into<String>) -> Result<Self> {
        let url = url.into();
        parse_url(&url)?;
        Ok(Self(url))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parsed(&self) -> Url {
        parse_url(&self.0).expect("RelayServiceUrl was validated on creation")
    }
}

impl FromStr for RelayServiceUrl {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for RelayServiceUrl {
    type Error = CommonError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<RelayServiceUrl> for String {
    fn from(value: RelayServiceUrl) -> Self {
        value.0
    }
}

impl Serialize for RelayServiceUrl {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for RelayServiceUrl {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let url = String::deserialize(deserializer)?;
        Self::new(url).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayService {
    /// User-facing label of the relay service.
    pub name: String,
    /// Base URL of relay API, e.g. https://radix-connect-relay.radixdlt.com/api/v1
    pub url: RelayServiceUrl,
}

impl fmt::Display for RelayService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.url)
    }
}

impl RelayService {
    pub fn new(name: impl Into<String>, url: RelayServiceUrl) -> Self {
        Self {
            name: name.into(),
            url,
        }
    }

    pub fn parse(
        name: impl Into<String>,
        url: impl Into<String>,
    ) -> Result<Self> {
        Ok(Self::new(name, RelayServiceUrl::new(url)?))
    }
}

impl Identifiable for RelayService {
    type ID = Url;

    fn id(&self) -> Url {
        self.url.parsed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedRelayServices {
    pub current: RelayService,
    #[serde(default)]
    pub other: RelayServices,
}

impl SavedRelayServices {
    pub fn new(current: RelayService) -> Self {
        Self {
            current,
            other: RelayServices::default(),
        }
    }

    pub fn all(&self) -> Vec<RelayService> {
        let mut all = Vec::new();
        all.push(self.current.clone());
        all.append(&mut self.other.items());
        all
    }

    pub fn has_url(&self, url: Url) -> bool {
        self.all().into_iter().any(|s| s.id() == url)
    }

    pub fn append(&mut self, service: RelayService) -> bool {
        if self.current.id() == service.id()
            || self.other.contains_by_id(&service)
        {
            return false;
        }
        self.other.try_insert_unique(service).is_ok()
    }

    pub fn remove(&mut self, service: &RelayService) -> bool {
        self.other.remove_id(&service.id()).is_some()
    }

    pub fn change_current(&mut self, to: RelayService) -> bool {
        if self.current.id() == to.id() {
            return false;
        }

        let old_current = self.current.clone();
        self.other.remove_id(&to.id());
        self.current = to;

        if !self.other.contains_by_id(&old_current) {
            let _ = self.other.try_insert_unique(old_current);
        }
        true
    }
}

impl Default for SavedRelayServices {
    fn default() -> Self {
        Self {
            current: RelayService::default_production_service(),
            other: RelayServices::default(),
        }
    }
}

impl RelayService {
    fn default_production_service() -> Self {
        Self::parse(
            "Radix Relay Production",
            "https://radix-connect-relay.radixdlt.com/api/v1",
        )
        .expect("valid URL")
    }
}

impl HasSampleValues for RelayService {
    fn sample() -> Self {
        Self::default_production_service()
    }

    fn sample_other() -> Self {
        Self::parse(
            "Sample Relay Alternate",
            "https://relay-alt.example/api/v1",
        )
        .expect("valid URL")
    }
}

impl HasSampleValues for SavedRelayServices {
    fn sample() -> Self {
        Self::default()
    }

    fn sample_other() -> Self {
        Self {
            current: RelayService::sample_other(),
            other: RelayServices::from_iter([RelayService::sample()]),
        }
    }
}

impl HasSampleValues for RelayServices {
    fn sample() -> Self {
        Self::from_iter([RelayService::sample_other()])
    }

    fn sample_other() -> Self {
        Self::from_iter([RelayService::sample()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SavedRelayServices;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn change_current_swaps_current_into_other() {
        let mut sut = SUT::default();
        let production = sut.current.clone();
        let development = RelayService::sample_other();

        assert!(sut.change_current(development.clone()));
        assert_eq!(sut.current, development);
        assert!(sut.other.contains_by_id(&production));
    }

    #[test]
    fn relay_service_preserves_exact_url_string() {
        let sut = RelayService::parse("Custom Relay", "https://relay.example")
            .unwrap();

        assert_eq!(sut.url.as_str(), "https://relay.example");
        assert_eq!(
            serde_json::to_value(&sut).unwrap(),
            json!({
                "name": "Custom Relay",
                "url": "https://relay.example"
            })
        );
    }

    #[test]
    fn relay_service_identity_still_treats_root_urls_as_equal() {
        let without_trailing_slash =
            RelayService::parse("Custom Relay", "https://relay.example")
                .unwrap();
        let with_trailing_slash =
            RelayService::parse("Custom Relay", "https://relay.example/")
                .unwrap();

        assert_eq!(without_trailing_slash.id(), with_trailing_slash.id());
    }

    #[test]
    fn relay_service_url_deserialization_still_validates() {
        assert!(
            serde_json::from_value::<RelayServiceUrl>(json!("not a url"))
                .is_err()
        );
    }
}
