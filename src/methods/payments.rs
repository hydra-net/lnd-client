use crate::errors::Error;
use crate::protobufs::{lnrpc, routerrpc};
use crate::LndClient;
use tonic::Streaming;

impl LndClient {
    pub async fn decode_payment_request(
        &mut self,
        payment_request: &str,
    ) -> Result<lnrpc::PayReq, Error> {
        let request = tonic::Request::new(lnrpc::PayReqString {
            pay_req: payment_request.to_string(),
        });

        let response = self
            .lightning_rpc
            .decode_pay_req(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn delete_payment(&mut self, payment_hash: &str) -> Result<(), Error> {
        let request = tonic::Request::new(lnrpc::DeletePaymentRequest {
            payment_hash: hex::decode(payment_hash)?,
            ..Default::default()
        });

        self.lightning_rpc.delete_payment(request).await?;

        Ok(())
    }

    pub async fn delete_all_payments(&mut self) -> Result<(), Error> {
        let request = tonic::Request::new(lnrpc::DeleteAllPaymentsRequest::default());

        self.lightning_rpc.delete_all_payments(request).await?;

        Ok(())
    }

    pub async fn delete_failed_payments(&mut self) -> Result<(), Error> {
        let request = tonic::Request::new(lnrpc::DeleteAllPaymentsRequest {
            failed_payments_only: true,
            ..Default::default()
        });

        self.lightning_rpc.delete_all_payments(request).await?;

        Ok(())
    }

    pub async fn delete_failed_pay_attempts(&mut self) -> Result<(), Error> {
        let request = tonic::Request::new(lnrpc::DeleteAllPaymentsRequest {
            failed_htlcs_only: true,
            ..Default::default()
        });

        self.lightning_rpc.delete_all_payments(request).await?;

        Ok(())
    }

    pub async fn get_payment(&mut self, payment_hash: &str) -> Result<lnrpc::Payment, Error> {
        let request = tonic::Request::new(routerrpc::TrackPaymentRequest {
            payment_hash: hex::decode(payment_hash)?,
            ..Default::default()
        });

        let mut response = self
            .router_rpc
            .track_payment_v2(request)
            .await?
            .into_inner();

        let payment = response.message().await?.unwrap_or_default();

        Ok(payment)
    }

    pub async fn get_payments(&mut self) -> Result<Vec<lnrpc::Payment>, Error> {
        let request = tonic::Request::new(lnrpc::ListPaymentsRequest {
            include_incomplete: true,
            ..Default::default()
        });

        let response = self
            .lightning_rpc
            .list_payments(request)
            .await?
            .into_inner();

        Ok(response.payments)
    }

    pub async fn get_completed_payments(&mut self) -> Result<Vec<lnrpc::Payment>, Error> {
        let request = tonic::Request::new(lnrpc::ListPaymentsRequest::default());

        let response = self
            .lightning_rpc
            .list_payments(request)
            .await?
            .into_inner();

        Ok(response.payments)
    }

    pub async fn get_failed_payments(&mut self) -> Result<Vec<lnrpc::Payment>, Error> {
        let request = tonic::Request::new(lnrpc::ListPaymentsRequest {
            include_incomplete: true,
            ..Default::default()
        });

        let response = self
            .lightning_rpc
            .list_payments(request)
            .await?
            .into_inner();

        let failed_payments = response
            .payments
            .into_iter()
            .filter(|payment| payment.status() == lnrpc::payment::PaymentStatus::Failed)
            .collect();

        Ok(failed_payments)
    }

    pub async fn get_pending_payments(&mut self) -> Result<Vec<lnrpc::Payment>, Error> {
        let request = tonic::Request::new(lnrpc::ListPaymentsRequest {
            include_incomplete: true,
            ..Default::default()
        });

        let response = self
            .lightning_rpc
            .list_payments(request)
            .await?
            .into_inner();

        let pending_payments = response
            .payments
            .into_iter()
            .filter(|payment| payment.status() == lnrpc::payment::PaymentStatus::InFlight)
            .collect();

        Ok(pending_payments)
    }

    pub async fn send_payment(
        &mut self,
        recipient: &str,
        amount: i64,
        timeout: Option<i32>,
    ) -> Result<Streaming<lnrpc::Payment>, Error> {
        let timeout = timeout.unwrap_or(60);

        let request = tonic::Request::new(routerrpc::SendPaymentRequest {
            dest: hex::decode(recipient)?,
            amt: amount,
            allow_self_payment: true,
            amp: true,
            timeout_seconds: timeout,
            ..Default::default()
        });

        let response = self.router_rpc.send_payment_v2(request).await?.into_inner();

        Ok(response)
    }

    pub async fn pay_invoice(
        &mut self,
        payment_request: &str,
        timeout: Option<i32>,
    ) -> Result<Streaming<lnrpc::Payment>, Error> {
        let timeout = timeout.unwrap_or(60);

        let request = tonic::Request::new(routerrpc::SendPaymentRequest {
            payment_request: payment_request.to_string(),
            allow_self_payment: true,
            amp: true,
            timeout_seconds: timeout,
            ..Default::default()
        });

        let response = self.router_rpc.send_payment_v2(request).await?.into_inner();

        Ok(response)
    }

    pub async fn pay_empty_invoice(
        &mut self,
        payment_request: &str,
        amount: i64,
        timeout: Option<i32>,
    ) -> Result<Streaming<lnrpc::Payment>, Error> {
        let timeout = timeout.unwrap_or(60);

        let request = tonic::Request::new(routerrpc::SendPaymentRequest {
            payment_request: payment_request.to_string(),
            allow_self_payment: true,
            amp: true,
            timeout_seconds: timeout,
            amt: amount,
            ..Default::default()
        });

        let response = self.router_rpc.send_payment_v2(request).await?.into_inner();

        Ok(response)
    }

    pub async fn send_to_route(
        &mut self,
        route: lnrpc::Route,
    ) -> Result<lnrpc::HtlcAttempt, Error> {
        let request = tonic::Request::new(routerrpc::SendToRouteRequest {
            route: Some(route),
            ..Default::default()
        });

        let response = self
            .router_rpc
            .send_to_route_v2(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn subscribe_payment(
        &mut self,
        payment_hash: &str,
    ) -> Result<Streaming<lnrpc::Payment>, Error> {
        let request = tonic::Request::new(routerrpc::TrackPaymentRequest {
            payment_hash: hex::decode(payment_hash)?,
            ..Default::default()
        });

        let response = self
            .router_rpc
            .track_payment_v2(request)
            .await?
            .into_inner();

        Ok(response)
    }
}
