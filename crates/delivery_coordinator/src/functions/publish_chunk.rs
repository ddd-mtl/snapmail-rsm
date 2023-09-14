use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;

/// Zome function
/// Write base64 data as string to source chain
/// Return EntryHash of newly created ParcelChunk
#[hdk_extern]
pub fn publish_chunk(data: String) -> ExternResult<EntryHash> {
   trace!("chunk size: {} bytes", data.len());
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Check size
   if data.is_empty() {
      return error("Data string is empty");
   }
   if data.len() > CHUNK_MAX_SIZE {
      return error(&format!("Data string is too big: {} > {}", data.len(), CHUNK_MAX_SIZE));
   }
   /// Commit entry
   let chunk = ParcelChunk {data};
   let chunk_eh = hash_entry(chunk.clone())?;
   let _chunk_ah = create_entry(DeliveryEntry::PublicChunk(chunk))?;
   /// Done
   Ok(chunk_eh)
}