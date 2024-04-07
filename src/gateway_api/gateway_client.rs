use crate::prelude::*;
use std::convert::identity;

const fn res_id<T>(x: T) -> Result<T, CommonError> {
    identity::<Result<T, CommonError>>(Ok(x))
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NetworkAntenna: Send + Sync {
    async fn make_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse, CommonError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum NetworkMethod {
    Post,
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NetworkRequest {
    pub url: Url,
    pub method: NetworkMethod,
    pub headers: HashMap<String, String>,

    pub body: BagOfBytes,
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NetworkResponse {
    pub status_code: u16,

    /// Can be empty.
    pub body: BagOfBytes,
}

#[derive(uniffi::Object)]
pub struct GatewayClient {
    pub network_antenna: Arc<dyn NetworkAntenna>,
    pub gateway: Gateway,
}

#[uniffi::export]
impl GatewayClient {
    #[uniffi::constructor]
    pub fn with_gateway(
        network_antenna: Arc<dyn NetworkAntenna>,
        gateway: Gateway,
    ) -> Self {
        Self {
            network_antenna,
            gateway,
        }
    }

    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
        network_id: NetworkID,
    ) -> Self {
        Self::with_gateway(network_antenna, Gateway::from(network_id))
    }
}

impl From<NetworkID> for Gateway {
    fn from(value: NetworkID) -> Self {
        match value {
            NetworkID::Mainnet => Self::mainnet(),
            NetworkID::Stokenet => Self::stokenet(),
            NetworkID::Nebunet => Self::nebunet(),
            NetworkID::Kisharnet => Self::kisharnet(),
            NetworkID::Ansharnet => Self::ansharnet(),
            NetworkID::Enkinet => Self::enkinet(),
            NetworkID::Hammunet => Self::hammunet(),
            NetworkID::Mardunet => Self::mardunet(),
            NetworkID::Adapanet => todo!(),
            NetworkID::Zabanet => todo!(),
            NetworkID::Nergalnet => todo!(),
            NetworkID::Simulator => panic!("No network exists for simulator"),
        }
    }
}

impl GatewayClient {
    pub(crate) async fn state_entity_details(
        &self,
        request: StateEntityDetailsRequest,
    ) -> Result<StateEntityDetailsResponse> {
        self.post("state/entity/details", request, res_id).await
    }
}

#[uniffi::export]
impl GatewayClient {
    pub async fn xrd_balance_of_account(
        &self,
        address: AccountAddress,
    ) -> Result<Option<Decimal192>> {
        let response: StateEntityDetailsResponse = self
            .state_entity_details(StateEntityDetailsRequest {
                addresses: vec![address.into()],
            })
            .await?;

        let Some(response_item) = response
            .items
            .into_iter()
            .find(|x| x.address == address.into())
        else {
            return Ok(None);
        };

        let Some(fungible_resources) = response_item.fungible_resources else {
            return Ok(None);
        };

        let xrd_address = ResourceAddress::xrd_on_network(address.network_id());

        let Some(xrd_resource_collection_item) = fungible_resources
            .items
            .into_iter()
            .find(|x| x.resource_address() == xrd_address)
        else {
            return Ok(None);
        };

        let Some(xrd_resource) = xrd_resource_collection_item.as_global()
        else {
            return Ok(None);
        };

        Ok(Some(xrd_resource.amount))
    }

    pub async fn xrd_balance_of_account_or_zero(
        &self,
        address: AccountAddress,
    ) -> Result<Decimal192> {
        self.xrd_balance_of_account(address)
            .await
            .map(|x| x.unwrap_or(Decimal192::zero()))
    }
}

impl GatewayClient {
    fn model_from_response<U>(
        &self,
        response: NetworkResponse,
    ) -> Result<U, CommonError>
    where
        U: for<'a> Deserialize<'a>,
    {
        if let 200..=299 = response.status_code {
            // all good
        } else {
            return Err(CommonError::NetworkResponseBadCode);
        }

        let body = response.body;
        if body.is_empty() {
            return Err(CommonError::NetworkResponseEmptyBody);
        }

        serde_json::from_slice::<U>(&body).map_err(|_| {
            CommonError::NetworkResponseJSONDeserialize {
                into_type: std::any::type_name::<U>().to_owned(),
            }
        })
    }

    async fn make_request<T, U, V, F>(
        &self,
        path: impl AsRef<str>,
        method: NetworkMethod,
        request: T,
        map: F,
    ) -> Result<V, CommonError>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, CommonError>,
    {
        // JSON serialize request into body bytes
        let body = BagOfBytes::from(serde_json::to_vec(&request).unwrap());

        // Append relative path to base url
        let url_str = format!("{}{}", self.gateway.url, path.as_ref());
        let url = Url::parse(&url_str).map_err(|e| {
            error!(
                "Failed to parse URL, error: {:?}, from string: {}",
                e, &url_str
            );
            CommonError::NetworkRequestInvalidUrl {
                bad_value: url_str.to_owned(),
            }
        })?;
        let request = NetworkRequest {
            url,
            body,
            method,
            headers: HashMap::<String, String>::from_iter([
                ("content-Type".to_owned(), "application/json".to_owned()),
                ("accept".to_owned(), "application/json".to_owned()),
                ("user-agent".to_owned(), "Sargon".to_owned()), // https://stackoverflow.com/a/77866494/1311272
                ("RDX-Client-Name".to_owned(), "Sargon".to_owned()),
                ("RDX-Client-Version".to_owned(), "1.5.1".to_owned()),
            ]),
        };

        // Let Swift side make network request and await response
        let response = self.network_antenna.make_request(request).await?;

        // Read out HTTP body from response and JSON parse it into U
        let model = self.model_from_response(response)?;

        // Map U -> V
        map(model)
    }

    async fn post<T, U, V, F>(
        &self,
        path: impl AsRef<str>,
        request: T,
        map: F,
    ) -> Result<V, CommonError>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, CommonError>,
    {
        self.make_request(path, NetworkMethod::Post, request, map)
            .await
    }
}
