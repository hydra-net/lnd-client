use crate::errors::Error;
use crate::protobufs::{invoicesrpc, lnrpc};
use crate::LndClient;
use tonic::Streaming;

impl LndClient {
    pub async fn create_invoice(
        &mut self,
        amount: i64,
        expiry: Option<i64>,
    ) -> Result<lnrpc::AddInvoiceResponse, Error> {
        let request = tonic::Request::new(lnrpc::Invoice {
            value: amount,
            expiry: expiry.unwrap_or_default(),
            is_amp: true,
            ..Default::default()
        });

        let response = self.lightning_rpc.add_invoice(request).await?.into_inner();

        Ok(response)
    }

    pub async fn create_hold_invoice(
        &mut self,
        amount: i64,
        hash: &str,
        cltv_expiry: Option<u64>,
        expiry: Option<i64>,
    ) -> Result<invoicesrpc::AddHoldInvoiceResp, Error> {
        let request = tonic::Request::new(invoicesrpc::AddHoldInvoiceRequest {
            hash: hex::decode(hash)?,
            value: amount,
            expiry: expiry.unwrap_or_default(),
            cltv_expiry: cltv_expiry.unwrap_or_default(),
            ..Default::default()
        });

        let response = self
            .invoices_rpc
            .add_hold_invoice(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn cancel_invoice(&mut self, payment_hash: &str) -> Result<(), Error> {
        let request = tonic::Request::new(invoicesrpc::CancelInvoiceMsg {
            payment_hash: hex::decode(payment_hash)?,
        });

        self.invoices_rpc.cancel_invoice(request).await?;

        Ok(())
    }

    pub async fn get_invoice(&mut self, payment_hash: &str) -> Result<lnrpc::Invoice, Error> {
        let request = tonic::Request::new(invoicesrpc::LookupInvoiceMsg {
            invoice_ref: Some(invoicesrpc::lookup_invoice_msg::InvoiceRef::PaymentHash(
                hex::decode(payment_hash)?,
            )),
            ..Default::default()
        });

        let response = self
            .invoices_rpc
            .lookup_invoice_v2(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn get_invoices(&mut self) -> Result<Vec<lnrpc::Invoice>, Error> {
        let request = tonic::Request::new(lnrpc::ListInvoiceRequest::default());

        let response = self
            .lightning_rpc
            .list_invoices(request)
            .await?
            .into_inner();

        Ok(response.invoices)
    }

    pub async fn settle_invoice(&mut self, preimage: &str) -> Result<(), Error> {
        let request = tonic::Request::new(invoicesrpc::SettleInvoiceMsg {
            preimage: hex::decode(preimage)?,
        });

        self.invoices_rpc.settle_invoice(request).await?;

        Ok(())
    }

    pub async fn subscribe_invoices(&mut self) -> Result<Streaming<lnrpc::Invoice>, Error> {
        let request = tonic::Request::new(lnrpc::InvoiceSubscription::default());

        let response = self
            .lightning_rpc
            .subscribe_invoices(request)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn subscribe_invoice(
        &mut self,
        hash: &str,
    ) -> Result<Streaming<lnrpc::Invoice>, Error> {
        let request = tonic::Request::new(invoicesrpc::SubscribeSingleInvoiceRequest {
            r_hash: hex::decode(hash)?,
        });

        let response = self
            .invoices_rpc
            .subscribe_single_invoice(request)
            .await?
            .into_inner();

        Ok(response)
    }
}
