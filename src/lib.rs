mod errors;
pub mod protobufs;

use errors::Error;
use protobufs::{
    chainrpc::chain_notifier_client::ChainNotifierClient,
    invoicesrpc::invoices_client::InvoicesClient,
    lnrpc::{lightning_client::LightningClient, wallet_unlocker_client::WalletUnlockerClient},
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
    pub lightning_client: LightningRpcClient,
    pub wallet_unlocker_client: WalletUnlockerRpcClient,
    pub chain_notifier_client: ChainNotifierRpcClient,
    pub invoices_client: InvoicesRpcClient,
    pub router_client: RouterRpcClient,
}

impl LndClient {
    pub async fn new(
        node_url: String,
        cert_dir: String,
        macaroon_dir: String,
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

        let channel = Endpoint::from_shared(node_url)?
            .tls_config(tls)?
            .connect()
            .await?;

        let macaroon = hex::encode(std::fs::read(macaroon_dir)?);

        let macaroon_interceptor = MacaroonInterceptor { macaroon };

        let lightning_client =
            protobufs::lnrpc::lightning_client::LightningClient::with_interceptor(
                channel.to_owned(),
                macaroon_interceptor.to_owned(),
            );

        let wallet_unlocker_client =
            protobufs::lnrpc::wallet_unlocker_client::WalletUnlockerClient::with_interceptor(
                channel.to_owned(),
                macaroon_interceptor.to_owned(),
            );

        let chain_notifier_client =
            protobufs::chainrpc::chain_notifier_client::ChainNotifierClient::with_interceptor(
                channel.to_owned(),
                macaroon_interceptor.to_owned(),
            );

        let invoices_client =
            protobufs::invoicesrpc::invoices_client::InvoicesClient::with_interceptor(
                channel.to_owned(),
                macaroon_interceptor.to_owned(),
            );

        let router_client = protobufs::routerrpc::router_client::RouterClient::with_interceptor(
            channel.to_owned(),
            macaroon_interceptor.to_owned(),
        );

        let client = LndClient {
            lightning_client,
            wallet_unlocker_client,
            chain_notifier_client,
            invoices_client,
            router_client,
        };

        return Ok(client);
    }
}

pub(crate) struct CertVerifier {
    cert: Vec<u8>,
}

impl CertVerifier {
    pub(crate) async fn load(path: String) -> Result<Self, Error> {
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
