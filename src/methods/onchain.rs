use crate::errors::Error;
use crate::protobufs::lnrpc;
use crate::LndClient;
use std::collections::HashMap;
use tonic::Streaming;

impl LndClient {
    pub async fn open_channel_sync(
        &mut self,
        pubkey: &str,
        amount: u64,
        sat_per_vbyte: Option<u64>,
        target_conf: Option<i32>,
    ) -> Result<lnrpc::ChannelPoint, Error> {
        let request = tonic::Request::new(lnrpc::OpenChannelRequest {
            sat_per_vbyte: sat_per_vbyte.unwrap_or_default(),
            node_pubkey: hex::decode(&pubkey)?,
            local_funding_amount: amount as i64,
            target_conf: target_conf.unwrap_or_default(),
            ..Default::default()
        });

        let response = self
            .lightning_rpc
            .open_channel_sync(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn open_channel(
        &mut self,
        pubkey: &str,
        amount: u64,
        sat_per_vbyte: Option<u64>,
        target_conf: Option<i32>,
    ) -> Result<Streaming<lnrpc::OpenStatusUpdate>, Error> {
        let request = tonic::Request::new(lnrpc::OpenChannelRequest {
            sat_per_vbyte: sat_per_vbyte.unwrap_or_default(),
            node_pubkey: hex::decode(&pubkey)?,
            local_funding_amount: amount as i64,
            target_conf: target_conf.unwrap_or_default(),
            ..Default::default()
        });

        let response = self.lightning_rpc.open_channel(request).await?.into_inner();

        Ok(response)
    }

    pub async fn close_channel(
        &mut self,
        funding_txid: &str,
        output_index: u32,
        force: bool,
        sat_per_vbyte: Option<u64>,
        target_conf: Option<i32>,
        delivery_address: Option<&str>,
    ) -> Result<Streaming<lnrpc::CloseStatusUpdate>, Error> {
        let request = tonic::Request::new(lnrpc::CloseChannelRequest {
            channel_point: Some(lnrpc::ChannelPoint {
                funding_txid: Some(lnrpc::channel_point::FundingTxid::FundingTxidStr(
                    funding_txid.to_owned(),
                )),
                output_index,
            }),
            force,
            target_conf: target_conf.unwrap_or_default(),
            delivery_address: delivery_address.unwrap_or_default().to_string(),
            sat_per_vbyte: sat_per_vbyte.unwrap_or_default(),
            ..Default::default()
        });

        let response = self
            .lightning_rpc
            .close_channel(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn cancel_pending_channel(&mut self, pending_channel_id: &str) -> Result<(), Error> {
        let request = tonic::Request::new(lnrpc::FundingTransitionMsg {
            trigger: Some(lnrpc::funding_transition_msg::Trigger::ShimCancel(
                lnrpc::FundingShimCancel {
                    pending_chan_id: hex::decode(pending_channel_id)?,
                },
            )),
        });

        self.lightning_rpc.funding_state_step(request).await?;

        Ok(())
    }

    pub async fn get_onchain_balance(&mut self) -> Result<lnrpc::WalletBalanceResponse, Error> {
        let request = tonic::Request::new(lnrpc::WalletBalanceRequest {});

        let response = self
            .lightning_rpc
            .wallet_balance(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn estimate_onchain_fee(
        &mut self,
        address: &str,
        amount: i64,
        target_conf: i32,
    ) -> Result<lnrpc::EstimateFeeResponse, Error> {
        let mut addr_to_amount = HashMap::new();
        addr_to_amount.insert(address.to_string(), amount);

        let request = tonic::Request::new(lnrpc::EstimateFeeRequest {
            addr_to_amount,
            target_conf,
            ..Default::default()
        });

        let response = self.lightning_rpc.estimate_fee(request).await?.into_inner();

        Ok(response)
    }

    pub async fn get_onchain_transactions(&mut self) -> Result<Vec<lnrpc::Transaction>, Error> {
        let request = tonic::Request::new(lnrpc::GetTransactionsRequest {
            start_height: 0,
            end_height: -1, // include unconfirmed txns
            ..Default::default()
        });

        let response = self
            .lightning_rpc
            .get_transactions(request)
            .await?
            .into_inner();

        Ok(response.transactions)
    }

    pub async fn get_utxos(&mut self) -> Result<Vec<lnrpc::Utxo>, Error> {
        let request = tonic::Request::new(lnrpc::ListUnspentRequest::default());

        let response = self.lightning_rpc.list_unspent(request).await?.into_inner();

        Ok(response.utxos)
    }

    pub async fn send_coins(
        &mut self,
        address: &str,
        amount: i64,
        sat_per_vbyte: Option<u64>,
        target_conf: Option<i32>,
    ) -> Result<String, Error> {
        let request = tonic::Request::new(lnrpc::SendCoinsRequest {
            addr: address.to_string(),
            amount,
            target_conf: target_conf.unwrap_or_default(),
            sat_per_vbyte: sat_per_vbyte.unwrap_or_default(),
            ..Default::default()
        });

        let response = self.lightning_rpc.send_coins(request).await?.into_inner();

        Ok(response.txid)
    }

    pub async fn send_many(
        &mut self,
        addresses: Vec<&str>,
        amounts: Vec<i64>,
        sat_per_vbyte: Option<u64>,
        target_conf: Option<i32>,
    ) -> Result<String, Error> {
        let mut addr_to_amount = HashMap::new();
        for (address, amount) in addresses.iter().zip(amounts.iter()) {
            addr_to_amount.insert(address.to_string(), *amount);
        }

        let request = tonic::Request::new(lnrpc::SendManyRequest {
            addr_to_amount,
            target_conf: target_conf.unwrap_or_default(),
            sat_per_vbyte: sat_per_vbyte.unwrap_or_default(),
            ..Default::default()
        });

        let response = self.lightning_rpc.send_many(request).await?.into_inner();

        Ok(response.txid)
    }

    pub async fn subscribe_onchain_transactions(
        &mut self,
    ) -> Result<Streaming<lnrpc::Transaction>, Error> {
        let request = tonic::Request::new(lnrpc::GetTransactionsRequest {
            start_height: 0,
            end_height: -1, // include unconfirmed txns
            ..Default::default()
        });

        let response = self
            .lightning_rpc
            .subscribe_transactions(request)
            .await?
            .into_inner();

        Ok(response)
    }
}
