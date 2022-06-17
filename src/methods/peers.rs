use crate::errors::Error;
use crate::protobufs::lnrpc;
use crate::LndClient;
use tonic::Streaming;

impl LndClient {
    pub async fn connect_peer(&mut self, pubkey_host: &str) -> Result<(), Error> {
        let split = pubkey_host.split('@').collect::<Vec<&str>>();

        let request = tonic::Request::new(lnrpc::ConnectPeerRequest {
            addr: Some(lnrpc::LightningAddress {
                pubkey: split[0].to_string(),
                host: split[1].to_string(),
            }),
            perm: true,
            timeout: 30,
        });

        self.lightning_rpc.connect_peer(request).await?;

        Ok(())
    }

    pub async fn disconnect_peer(&mut self, pubkey: &str) -> Result<(), Error> {
        let request = tonic::Request::new(lnrpc::DisconnectPeerRequest {
            pub_key: pubkey.to_string(),
        });

        self.lightning_rpc.disconnect_peer(request).await?;

        Ok(())
    }

    pub async fn list_peers(&mut self) -> Result<Vec<lnrpc::Peer>, Error> {
        let request = tonic::Request::new(lnrpc::ListPeersRequest {
            latest_error: false,
        });

        let response = self.lightning_rpc.list_peers(request).await?.into_inner();

        Ok(response.peers)
    }

    pub async fn subscribe_peer_events(&mut self) -> Result<Streaming<lnrpc::PeerEvent>, Error> {
        let request = tonic::Request::new(lnrpc::PeerEventSubscription {});

        let response = self
            .lightning_rpc
            .subscribe_peer_events(request)
            .await?
            .into_inner();

        Ok(response)
    }
}
