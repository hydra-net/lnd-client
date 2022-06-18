use crate::errors::Error;
use crate::protobufs::{lnrpc, routerrpc};
use crate::LndClient;
use tonic::Streaming;

impl LndClient {
    pub async fn get_channels(&mut self) -> Result<Vec<lnrpc::Channel>, Error> {
        let request = tonic::Request::new(lnrpc::ListChannelsRequest::default());

        let response = self
            .lightning_rpc
            .list_channels(request)
            .await?
            .into_inner();

        Ok(response.channels)
    }

    pub async fn get_active_channels(&mut self) -> Result<Vec<lnrpc::Channel>, Error> {
        let request = tonic::Request::new(lnrpc::ListChannelsRequest {
            active_only: true,
            ..Default::default()
        });

        let response = self
            .lightning_rpc
            .list_channels(request)
            .await?
            .into_inner();

        Ok(response.channels)
    }

    pub async fn get_inactive_channels(&mut self) -> Result<Vec<lnrpc::Channel>, Error> {
        let request = tonic::Request::new(lnrpc::ListChannelsRequest {
            inactive_only: true,
            ..Default::default()
        });

        let response = self
            .lightning_rpc
            .list_channels(request)
            .await?
            .into_inner();

        Ok(response.channels)
    }

    pub async fn get_pending_channels(&mut self) -> Result<lnrpc::PendingChannelsResponse, Error> {
        let request = tonic::Request::new(lnrpc::PendingChannelsRequest {});

        let response = self
            .lightning_rpc
            .pending_channels(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn get_closed_channels(&mut self) -> Result<Vec<lnrpc::ChannelCloseSummary>, Error> {
        let request = tonic::Request::new(lnrpc::ClosedChannelsRequest::default());

        let response = self
            .lightning_rpc
            .closed_channels(request)
            .await?
            .into_inner();

        Ok(response.channels)
    }

    pub async fn disable_channel(
        &mut self,
        funding_txid: &str,
        output_index: u32,
    ) -> Result<(), Error> {
        let channel_point = lnrpc::ChannelPoint {
            funding_txid: Some(lnrpc::channel_point::FundingTxid::FundingTxidStr(
                funding_txid.to_string(),
            )),
            output_index: output_index,
        };

        let request = tonic::Request::new(routerrpc::UpdateChanStatusRequest {
            chan_point: Some(channel_point),
            action: routerrpc::ChanStatusAction::Disable.into(),
        });

        self.router_rpc.update_chan_status(request).await?;

        Ok(())
    }

    pub async fn enable_channel(
        &mut self,
        funding_txid: &str,
        output_index: u32,
    ) -> Result<(), Error> {
        let channel_point = lnrpc::ChannelPoint {
            funding_txid: Some(lnrpc::channel_point::FundingTxid::FundingTxidStr(
                funding_txid.to_string(),
            )),
            output_index: output_index,
        };

        let request = tonic::Request::new(routerrpc::UpdateChanStatusRequest {
            chan_point: Some(channel_point),
            action: routerrpc::ChanStatusAction::Enable.into(),
        });

        self.router_rpc.update_chan_status(request).await?;

        Ok(())
    }

    pub async fn get_offchain_balance(&mut self) -> Result<lnrpc::ChannelBalanceResponse, Error> {
        let request = tonic::Request::new(lnrpc::ChannelBalanceRequest {});

        let response = self
            .lightning_rpc
            .channel_balance(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn subscribe_channels(
        &mut self,
    ) -> Result<Streaming<lnrpc::ChannelEventUpdate>, Error> {
        let request = tonic::Request::new(lnrpc::ChannelEventSubscription {});

        let response = self
            .lightning_rpc
            .subscribe_channel_events(request)
            .await?
            .into_inner();

        Ok(response)
    }
}
