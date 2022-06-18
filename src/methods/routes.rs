use crate::errors::Error;
use crate::protobufs::{lnrpc, routerrpc};
use crate::LndClient;

impl LndClient {
    pub async fn estimate_route_fee(
        &mut self,
        recipient: &str,
        amount: i64,
    ) -> Result<routerrpc::RouteFeeResponse, Error> {
        let request = tonic::Request::new(routerrpc::RouteFeeRequest {
            dest: hex::decode(recipient)?,
            amt_sat: amount,
        });

        let response = self
            .router_rpc
            .estimate_route_fee(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn estimate_route_probability(
        &mut self,
        sender_pubkey: &str,
        destination_pubkey: &str,
        amount: i64,
    ) -> Result<routerrpc::QueryProbabilityResponse, Error> {
        let request = tonic::Request::new(routerrpc::QueryProbabilityRequest {
            from_node: hex::decode(sender_pubkey)?,
            to_node: hex::decode(destination_pubkey)?,
            amt_msat: amount * 1000,
        });

        let response = self
            .router_rpc
            .query_probability(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn get_routes(
        &mut self,
        recipient: &str,
        amount: i64,
    ) -> Result<lnrpc::QueryRoutesResponse, Error> {
        let request = tonic::Request::new(lnrpc::QueryRoutesRequest {
            pub_key: recipient.to_string(),
            amt: amount,
            ..Default::default()
        });

        let response = self.lightning_rpc.query_routes(request).await?.into_inner();

        Ok(response)
    }

    pub async fn build_route(
        &mut self,
        hop_pubkeys: &[&str],
        amount: i64,
    ) -> Result<Option<lnrpc::Route>, Error> {
        let mut hop_pubkey_bytes = vec![];

        for hop_pubkey in hop_pubkeys {
            hop_pubkey_bytes.push(hex::decode(hop_pubkey)?);
        }

        let request = tonic::Request::new(routerrpc::BuildRouteRequest {
            hop_pubkeys: hop_pubkey_bytes,
            amt_msat: amount * 1000,
            ..Default::default()
        });

        let response = self.router_rpc.build_route(request).await?.into_inner();

        Ok(response.route)
    }
}
