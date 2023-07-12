use hdk::prelude::*;
use zome_delivery_types::*;
use zome_delivery_integrity::*;
use zome_utils::*;

use crate::send_item::*;
use crate::functions::*;

///
pub fn post_commit_ReplyReceived(entry: Entry, reply_eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_ReplyReceived() {:?}", reply_eh);
   let _ = ReplyReceived::try_from(entry)?;

   /// Get ReplyReceived
   let reply: ReplyReceived = get_typed_from_eh(reply_eh.clone())?;
   /// Check signature
   // FIXME
   /// Bail if delivery refused
   if !reply.has_accepted {
      info!("Delivery {} refused by {}", reply.distribution_eh, reply.recipient);
      return Ok(());
   }
   /// - Send Parcel
   /// Get Distribution
   let distribution: Distribution = get_typed_from_eh(reply.distribution_eh.clone())?;
   /// - Send Chunks if Manifest
   if let ParcelReference::Manifest(manifest_eh) = distribution.delivery_summary.parcel_reference.clone() {
      /// Get manifest
      let manifest: ParcelManifest = get_typed_from_eh(manifest_eh.clone())?;
      /// pack each chunk
      for chunk_eh in manifest.chunks {
         /// Get chunk
         let chunk: ParcelChunk = get_typed_from_eh(chunk_eh.clone())?;
         /// Create PendingItem
         let pending_item = pack_chunk(
            chunk,
            reply.distribution_eh.clone(),
            reply.recipient.clone(),
         )?;
         /// Send it to recipient
         debug!("Delivery accepted ; sending chunk {}", chunk_eh);
         let _ = send_item(
            reply.recipient.clone(),
            pending_item,
            distribution.delivery_summary.distribution_strategy.clone(),
         )?;
      }
   }
   /// - Send AppEntry Parcel
   debug!("Delivery accepted ; sending item {:?}", distribution.delivery_summary.parcel_reference);
   /// Get Entry
   let record = get_local_from_eh(distribution.delivery_summary.parcel_reference.entry_address().clone())?;
   let entry = record.entry().clone().into_option().unwrap();
   /// Create PendingItem
   let pending_item = pack_entry(
      entry,
      reply.distribution_eh.clone(),
      reply.recipient.clone(),
   )?;
   /// Send it to recipient
   let success = send_item(
      reply.recipient,
      pending_item,
      distribution.delivery_summary.distribution_strategy,
   )?;
   debug!("Delivery accepted ; sending result: {:?}", success);
   /// Done
   Ok(())
}
