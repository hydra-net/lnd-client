use std::str::FromStr;

use crate::errors::Error;
use crate::protobufs::lnrpc;
use crate::{CertVerifier, LndClient};
use tonic::transport::{ClientTlsConfig, Endpoint};
use tonic::Streaming;

impl LndClient {
    pub async fn init_wallet(
        node_url: &str,
        cert_dir: &str,
        mnemonic: Option<&str>,
        encryption_passphrase: Option<&str>,
        master_key: Option<&str>,
        birthdate: Option<u64>,
        password: &str,
    ) -> Result<Self, Error> {
        let cert_verifier = std::sync::Arc::new(CertVerifier::load(cert_dir).await?);
        let mut tls_config = rustls::ClientConfig::new();
        tls_config
            .dangerous()
            .set_certificate_verifier(cert_verifier);
        tls_config.set_protocols(&["h2".into()]);

        let tls = ClientTlsConfig::new().rustls_client_config(tls_config);
        let endpoint = Endpoint::from_str(node_url)?.tls_config(tls)?;

        let mut wallet_unlocker_client =
            lnrpc::wallet_unlocker_client::WalletUnlockerClient::connect(endpoint.to_owned())
                .await?;

        let mnemonic = match mnemonic {
            Some(mnemonic) => mnemonic.split(" ").map(|s| s.to_string()).collect(),
            None => vec![],
        };

        let request = lnrpc::InitWalletRequest {
            wallet_password: password.as_bytes().to_vec(),
            cipher_seed_mnemonic: mnemonic,
            aezeed_passphrase: encryption_passphrase
                .unwrap_or_default()
                .as_bytes()
                .to_vec(),
            extended_master_key: master_key.unwrap_or_default().to_string(),
            extended_master_key_birthday_timestamp: birthdate.unwrap_or_default(),
            ..Default::default()
        };

        let response = wallet_unlocker_client
            .init_wallet(request)
            .await?
            .into_inner();

        let macaroon = hex::encode(response.admin_macaroon);

        let client = crate::create_client(endpoint, macaroon).await?;

        Ok(client)
    }

    pub async fn unlock_wallet(&mut self, password: &str) -> Result<(), Error> {
        self.wallet_unlocker_rpc
            .unlock_wallet(lnrpc::UnlockWalletRequest {
                wallet_password: password.as_bytes().to_vec(),
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    pub async fn change_password(
        &mut self,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), Error> {
        self.wallet_unlocker_rpc
            .change_password(lnrpc::ChangePasswordRequest {
                current_password: current_password.as_bytes().to_vec(),
                new_password: new_password.as_bytes().to_vec(),
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    pub async fn get_wallet_status(&mut self) -> Result<lnrpc::WalletState, Error> {
        let response = self
            .state_rpc
            .get_state(lnrpc::GetStateRequest {})
            .await?
            .into_inner();

        let wallet_status: lnrpc::WalletState;
        match response.state {
            1 => wallet_status = lnrpc::WalletState::Locked,
            2 => wallet_status = lnrpc::WalletState::Unlocked,
            3 => wallet_status = lnrpc::WalletState::RpcActive,
            4 => wallet_status = lnrpc::WalletState::ServerActive,
            255 => wallet_status = lnrpc::WalletState::WaitingToStart,
            _ => wallet_status = lnrpc::WalletState::NonExisting,
        }

        Ok(wallet_status)
    }

    pub async fn subscribe_wallet_status(
        &mut self,
    ) -> Result<Streaming<lnrpc::SubscribeStateResponse>, Error> {
        let request = tonic::Request::new(lnrpc::SubscribeStateRequest {});

        let response = self.state_rpc.subscribe_state(request).await?.into_inner();

        Ok(response)
    }
}
