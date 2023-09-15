use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;


/// Zome function
/// Write base64 data as string to source chain
/// Return EntryHash of newly created ParcelChunk
#[hdk_extern]
pub fn commit_parcel_chunk(data: String) -> ExternResult<EntryHash> {
   trace!("chunk size: {} bytes", data.len());
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Commit entry
   let chunk = ParcelChunk {data};
   let chunk_eh = hash_entry(chunk.clone())?;
   let _chunk_ah = create_entry(DeliveryEntry::ParcelChunk(chunk))?;
   /// Done
   Ok(chunk_eh)
}
