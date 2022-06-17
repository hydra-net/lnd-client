use crate::errors::Error;
use crate::protobufs::lnrpc;
use crate::LndClient;

impl LndClient {
    pub async fn new_address(
        &mut self,
        address_type: lnrpc::AddressType,
    ) -> Result<lnrpc::NewAddressResponse, Error> {
        let request = tonic::Request::new(lnrpc::NewAddressRequest {
            r#type: address_type.into(),
            ..Default::default()
        });

        let response = self.lightning_rpc.new_address(request).await?.into_inner();

        Ok(response)
    }
}
