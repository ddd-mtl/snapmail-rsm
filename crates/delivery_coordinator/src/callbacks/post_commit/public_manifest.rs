use hdk::prelude::*;
use zome_delivery_types::*;
use crate::SignalProtocol;


///
pub fn post_commit_PublicManifest(sah: &SignedActionHashed, entry: Entry, manifest_eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_PublicManifest() {:?}", manifest_eh);
   let manifest = ParcelManifest::try_from(entry)?;
   let pr = ParcelReference {
      eh: manifest_eh.clone(),
      description: manifest.description.clone(),
   };
   /// Emit signal
   let res = emit_signal(&SignalProtocol::NewPublicParcel((sah.hashed.content.timestamp(), pr, agent_info()?.agent_latest_pubkey)));
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(())
}