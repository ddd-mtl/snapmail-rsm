use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
//use crate::zome_entry_trait::*;
use crate::send_item::*;
use crate::functions::*;

///
fn post_commit_Distribution(&self, distribution_eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_distribution() {:?}", distribution_eh);
    /// Create DeliveryNotice
    let notice = DeliveryNotice {
        distribution_eh: distribution_eh.clone(),
        sender: agent_info()?.agent_latest_pubkey,
        summary: self.delivery_summary.clone(),
        sender_summary_signature: self.summary_signature.clone(),
    };
    /// Send to each recipient
    for recipient in self.recipients.clone() {
        /// Create PendingItem
        let pending_item = pack_notice(
            notice.clone(),
            recipient.clone(),
        )?;
        /// Send it to recipient
        let res = send_item(
            recipient,
            pending_item,
            self.delivery_summary.distribution_strategy.clone(),
            // signature.clone(),
        );
        /// FIXME: accumulate failed recipients to final error return value
        if let Err(e) = res {
            warn!("send_item() during Distribution::post_commit() failed: {}", e);
        }
    }
    /// Done
    Ok(())
}
