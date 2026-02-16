use napi::bindgen_prelude::*;
use napi_derive::napi;
use sentinel_client::{ConnectionStream, SentinelClient};
use sentinel_shared::TransactionPayload;
use std::sync::Arc;
use tokio::sync::Mutex;

#[napi]
pub struct Client {
    inner: Arc<Mutex<SentinelClient<Box<dyn ConnectionStream>>>>,
}

#[napi]
impl Client {
    /// Connect using VSock (Linux) or fallback
    #[napi(factory)]
    pub async fn connect(cid: u32, port: u32) -> Result<Client> {
        let client = SentinelClient::connect(cid, port)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(Client {
            inner: Arc::new(Mutex::new(client)),
        })
    }

    /// Connect using TCP (Simulation/Mac)
    #[napi]
    pub async fn connect_tcp(addr: String) -> Result<Client> {
        let client = SentinelClient::connect_tcp(&addr)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(Client {
            inner: Arc::new(Mutex::new(client)),
        })
    }

    /// Sign a transaction
    ///
    /// @param to - Hex string (0x...) 20 bytes
    /// @param amount - String (to avoid JS Number precision issues)
    /// @param usd_value - Number (optional, default 0)
    /// @param agent_id - String (optional)
    #[napi]
    pub async fn sign_transaction(
        &self,
        to: String,
        amount: String,
        usd_value: Option<u32>,
        agent_id: Option<String>,
    ) -> Result<String> {
        let to_clean = to.strip_prefix("0x").unwrap_or(&to);
        let to_bytes =
            hex::decode(to_clean).map_err(|_| Error::from_reason("Invalid hex address"))?;

        if to_bytes.len() != 20 {
            return Err(Error::from_reason("Address must be 20 bytes"));
        }

        let mut to_arr = [0u8; 20];
        to_arr.copy_from_slice(&to_bytes);

        let amt = amount
            .parse::<u64>()
            .map_err(|_| Error::from_reason("Invalid amount integer"))?;

        let payload = TransactionPayload {
            to: to_arr,
            amount: amt,
            usd_value: usd_value.unwrap_or(0) as u64,
            agent_id,
        };

        let mut client = self.inner.lock().await;
        let sig = client
            .sign_transaction(payload)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(hex::encode(sig))
    }
}
