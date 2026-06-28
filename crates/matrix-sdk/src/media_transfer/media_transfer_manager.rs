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
    /// Active media transfers keyed by their unique transfer identifier.
    pub transfers: DashMap<String, TransferHandle>
}
impl MediaTransferManager {
    /// Creates a new, empty media transfer manager.
    pub fn new() -> Self {
        Self {
            transfers: DashMap::new()
        }
    }
    /// Registers a new media transfer and returns its cancellation token.
    ///
    /// The returned token should be passed to the transfer task so it can
    /// respond to cancellation requests.
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
    /// Cancels the transfer with the given identifier and removes it from
    /// the active transfer registry.
    pub fn cancel_transfer(
        &self,
        transfer_id: &str,
    ) {
        if let Some((_, handle)) = self.transfers.remove(transfer_id) {
            handle.cancel_token.cancel();
        }
    }
    /// Marks the transfer as completed and removes it from the active
    /// transfer registry.
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
    /// Token used to cancel the associated media transfer.
    pub cancel_token: CancellationToken,
}