use dashmap::DashMap;
use tokio_util::sync::CancellationToken;

/// Manages active media transfers and their cancellation lifecycle.
///
/// Each transfer is identified by a unique transfer ID and is associated with
/// a `CancellationToken`. This allows ongoing upload/download operations to be
/// cancelled and ensures completed or cancelled transfers are removed from the
/// registry to avoid retaining stale state.
#[derive(Debug)]
pub struct MediaTransferManager {
    pub transfers: DashMap<String, TransferHandle>
}
impl MediaTransferManager {
    pub fn new() -> Self {
        Self {
            transfers: DashMap::new()
        }
    }

    pub fn start_transfer(
        &self,
        transfer_id: String,
    ) -> CancellationToken {
        let token = CancellationToken::new();

        self.transfers.insert(
            transfer_id,
            TransferHandle {
                cancel_token: token.clone(),
            },
        );

        token
    }

    pub fn cancel_transfer(
        &self,
        transfer_id: &String,
    ) {
        if let Some((_, handle)) = self.transfers.remove(transfer_id) {
            handle.cancel_token.cancel();
        }
    }

    pub fn finish_transfer(
        &self,
        transfer_id: &str,
    ) {
        self.transfers.remove(transfer_id);
    }
}
/// Holds state associated with an active media transfer.
#[derive(Debug)]
pub struct TransferHandle {
    pub cancel_token: CancellationToken,
}