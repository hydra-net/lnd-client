mod errors;
mod methods;
pub mod protobufs;

use errors::Error;
use protobufs::{
    chainrpc::chain_notifier_client::ChainNotifierClient,
    invoicesrpc::invoices_client::InvoicesClient,
    lnrpc::{
        lightning_client::LightningClient, state_client::StateClient,
        wallet_unlocker_client::WalletUnlockerClient,
    },
    routerrpc::router_client::RouterClient,
};
use rustls::{RootCertStore, ServerCertVerified, TLSError};
use std::str::FromStr;
use tonic::{
    codegen::InterceptedService,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Status,
};
use webpki::DNSNameRef;

pub type LightningRpcClient = LightningClient<InterceptedService<Channel, MacaroonInterceptor>>;
pub type WalletUnlockerRpcClient =
    WalletUnlockerClient<InterceptedService<Channel, MacaroonInterceptor>>;
pub type ChainNotifierRpcClient =
    ChainNotifierClient<InterceptedService<Channel, MacaroonInterceptor>>;
pub type InvoicesRpcClient = InvoicesClient<InterceptedService<Channel, MacaroonInterceptor>>;
pub type RouterRpcClient = RouterClient<InterceptedService<Channel, MacaroonInterceptor>>;
pub type StateRpcClient = StateClient<InterceptedService<Channel, MacaroonInterceptor>>;

#[derive(Clone)]
pub struct LndClient {
    // pub node_id: String,
    pub lightning_rpc: LightningRpcClient,
    pub wallet_unlocker_rpc: WalletUnlockerRpcClient,
    pub chain_notifier_rpc: ChainNotifierRpcClient,
    pub invoices_rpc: InvoicesRpcClient,
    pub router_rpc: RouterRpcClient,
    pub state_rpc: StateRpcClient,
}

impl LndClient {
    pub async fn new(node_url: &str, cert_dir: &str, macaroon_dir: &str) -> Result<Self, Error> {
        let cert_verifier = std::sync::Arc::new(CertVerifier::load(cert_dir).await?);
        let mut tls_config = rustls::ClientConfig::new();
        tls_config
            .dangerous()
            .set_certificate_verifier(cert_verifier);
        tls_config.set_protocols(&["h2".into()]);

        let tls = ClientTlsConfig::new().rustls_client_config(tls_config);
        let endpoint = Endpoint::from_str(node_url)?.tls_config(tls)?;

        let macaroon = hex::encode(std::fs::read(macaroon_dir)?);

        let client = create_client(endpoint, macaroon).await?;

        Ok(client)
    }
}

pub(crate) struct CertVerifier {
    cert: Vec<u8>,
}

impl CertVerifier {
    pub(crate) async fn load(path: &str) -> Result<Self, Error> {
        let contents = std::fs::read(&path)?;
        let mut reader = &*contents;

        let mut certs = rustls_pemfile::certs(&mut reader)?;

        if certs.len() != 1 {
            return Err(Error::GenericError("Too many certs".to_string()));
        }

        Ok(CertVerifier {
            cert: certs.swap_remove(0),
        })
    }
}

impl rustls::ServerCertVerifier for CertVerifier {
    fn verify_server_cert(
        &self,
        _roots: &RootCertStore,
        presented_certs: &[rustls::Certificate],
        _dns_name: DNSNameRef<'_>,
        _ocsp_response: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        if presented_certs.len() != 1 {
            return Err(TLSError::General(format!(
                "server sent {} certificates, expected one",
                presented_certs.len()
            )));
        }
        if presented_certs[0].0 != self.cert {
            return Err(TLSError::General(format!(
                "server certificates doesn't match ours"
            )));
        }
        Ok(ServerCertVerified::assertion())
    }
}

#[derive(Clone)]
pub struct MacaroonInterceptor {
    macaroon: String,
}

impl tonic::service::Interceptor for MacaroonInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        request.metadata_mut().insert(
            "macaroon",
            tonic::metadata::MetadataValue::from_str(&self.macaroon)
                .expect("hex produced non-ascii"),
        );
        Ok(request)
    }
}

pub(crate) async fn create_client(
    endpoint: Endpoint,
    macaroon: String,
) -> Result<LndClient, Error> {
    let channel = endpoint.connect().await?;
    let macaroon_interceptor = MacaroonInterceptor { macaroon };

    let lightning_rpc = protobufs::lnrpc::lightning_client::LightningClient::with_interceptor(
        channel.to_owned(),
        macaroon_interceptor.to_owned(),
    );

    let wallet_unlocker_rpc =
        protobufs::lnrpc::wallet_unlocker_client::WalletUnlockerClient::with_interceptor(
            channel.to_owned(),
            macaroon_interceptor.to_owned(),
        );

    let chain_notifier_rpc =
        protobufs::chainrpc::chain_notifier_client::ChainNotifierClient::with_interceptor(
            channel.to_owned(),
            macaroon_interceptor.to_owned(),
        );

    let invoices_rpc = protobufs::invoicesrpc::invoices_client::InvoicesClient::with_interceptor(
        channel.to_owned(),
        macaroon_interceptor.to_owned(),
    );

    let router_rpc = protobufs::routerrpc::router_client::RouterClient::with_interceptor(
        channel.to_owned(),
        macaroon_interceptor.to_owned(),
    );

    let state_rpc = protobufs::lnrpc::state_client::StateClient::with_interceptor(
        channel.to_owned(),
        macaroon_interceptor.to_owned(),
    );

    let client = LndClient {
        lightning_rpc,
        wallet_unlocker_rpc,
        chain_notifier_rpc,
        invoices_rpc,
        router_rpc,
        state_rpc,
    };

    Ok(client)
}
