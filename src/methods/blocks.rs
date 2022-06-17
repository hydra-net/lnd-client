use crate::errors::Error;
use crate::protobufs::chainrpc;
use crate::LndClient;
use tonic::Streaming;

impl LndClient {
    pub async fn get_latest_block(&mut self) -> Result<chainrpc::BlockEpoch, Error> {
        let request = tonic::Request::new(chainrpc::BlockEpoch::default());

        let mut response = self
            .chain_notifier_rpc
            .register_block_epoch_ntfn(request)
            .await?
            .into_inner();

        let block = response.message().await?.unwrap_or_default();

        Ok(block)
    }

    pub async fn subscribe_blocks(&mut self) -> Result<Streaming<chainrpc::BlockEpoch>, Error> {
        let request = tonic::Request::new(chainrpc::BlockEpoch::default());

        let response = self
            .chain_notifier_rpc
            .register_block_epoch_ntfn(request)
            .await?
            .into_inner();

        Ok(response)
    }
}
