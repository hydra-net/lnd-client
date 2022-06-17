mod errors;
pub mod protobufs;

use std::str::FromStr;

use errors::Error;
use protobufs::{
    chainrpc::chain_notifier_client::ChainNotifierClient,
    invoicesrpc::invoices_client::InvoicesClient,
    lnrpc::{
        lightning_client::LightningClient, wallet_unlocker_client::WalletUnlockerClient,
        InitWalletRequest, UnlockWalletRequest,
    },
    routerrpc::router_client::RouterClient,
};
use rustls::{RootCertStore, ServerCertVerified, TLSError};
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

#[derive(Clone)]
pub struct LndClient {
    // pub node_id: String,
    pub lightning_rpc: LightningRpcClient,
    pub wallet_unlocker_rpc: WalletUnlockerRpcClient,
    pub chain_notifier_rpc: ChainNotifierRpcClient,
    pub invoices_rpc: InvoicesRpcClient,
    pub router_rpc: RouterRpcClient,
}

impl LndClient {
    pub async fn init(
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
            protobufs::lnrpc::wallet_unlocker_client::WalletUnlockerClient::connect(
                endpoint.to_owned(),
            )
            .await?;

        let mnemonic = match mnemonic {
            Some(mnemonic) => mnemonic.split(" ").map(|s| s.to_string()).collect(),
            None => vec![],
        };

        let request = InitWalletRequest {
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

        let client = create_client(endpoint, macaroon).await?;

        Ok(client)
    }

    pub async fn new(
        node_url: &str,
        cert_dir: &str,
        macaroon_dir: &str,
        password: &str,
    ) -> Result<Self, Error> {
        let cert_verifier = std::sync::Arc::new(CertVerifier::load(cert_dir).await?);
        let mut tls_config = rustls::ClientConfig::new();
        tls_config
            .dangerous()
            .set_certificate_verifier(cert_verifier);
        tls_config.set_protocols(&["h2".into()]);

        // let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
        //     .with_tls_config(tls_config)
        //     .https_or_http()
        //     .enable_http2()
        //     .build();

        // let client = hyper::Client::builder().build(https_connector);

        // let uri = Uri::from_static("https://127.0.0.1:10080");
        // let svc = tower::ServiceBuilder::new()
        //     .map_request(|mut req: hyper::Request<tonic::body::BoxBody>| {
        //         println!("{}", req.uri());
        //         let uri = Uri::builder()
        //             .scheme(uri.scheme().unwrap().clone())
        //             .authority(uri.authority().unwrap().clone())
        //             .path_and_query(req.uri().path_and_query().unwrap().clone())
        //             .build()
        //             .unwrap();
        //         println!("{}", uri);
        //         *req.uri_mut() = uri;
        //         req
        //     })
        //     .service(client);

        let tls = ClientTlsConfig::new().rustls_client_config(tls_config);
        let endpoint = Endpoint::from_str(node_url)?.tls_config(tls)?;

        let macaroon = hex::encode(std::fs::read(macaroon_dir)?);

        let mut client = create_client(endpoint, macaroon).await?;

        client
            .wallet_unlocker_rpc
            .unlock_wallet(UnlockWalletRequest {
                wallet_password: password.as_bytes().to_vec(),
                ..Default::default()
            })
            .await?;

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

async fn create_client(endpoint: Endpoint, macaroon: String) -> Result<LndClient, Error> {
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

    let client = LndClient {
        lightning_rpc,
        wallet_unlocker_rpc,
        chain_notifier_rpc,
        invoices_rpc,
        router_rpc,
    };

    Ok(client)
}
