#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


#[macro_use]
extern crate zome_proc_macro;

use hdk::prelude::*;
use delivery::*;

/// Entry representing a secret
#[hdk_entry(id = "Secret", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct Secret {
   pub value: String,
}


/// Zome Function
#[hdk_extern]
pub fn create_secret(value: String) -> ExternResult<EntryHash> {
   let secret = Secret { value };
   let eh = hash_entry(secret.clone())?;
   let _hh = create_entry(secret)?;
   return Ok(eh);
}

/// Zome Function
#[hdk_extern]
pub fn get_secret(eh: EntryHash) -> ExternResult<String> {
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .entry_hashes([eh]);
   let entries = query(query_args)?;
   if entries.len() != 1 {
      return Err(WasmError::Guest(String::from("No Secret found at given EntryHash")));
   }
   let secret = get_typed_from_el(entries[0])?;
   return Ok(secret.value);
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendSecretInput {
   secret_eh: EntryHash,
   recipient: AgentPubKey,
}

/// Zome Function
#[hdk_extern]
pub fn send_secret(input: SendSecretInput) -> ExternResult<EntryHash> {
   /// Make sure secret is committed
   let _: Secret = get_typed_from_eh(input.secret_eh.clone())?;
   /// Distribute
   let distribution = DistributeParcelInput {
      recipients: [input.recipient],
      strategy: DistributionStrategy::NORMAL,
      parcel_kind: ParcelKind::AppEntry((zome_info()?.name, App("Secret"))),
      parcel_eh: input.secret_eh,
   };
   return distribute_parcel(distribution)?;
}

/// Zome Function
#[hdk_extern]
pub fn get_secrets_from(sender: AgentPubKey) -> ExternResult<Vec<EntryHash>> {
   let notices = DeliveryNotice::query(DeliveryNoticeQueryField::Sender(sender))?;
   let parcels = notices.iter().map(|x| x.parcel_summary.entry_address()).collect;
   Ok(parcels)
}


/// Zome Function
#[hdk_extern]
pub fn accept_secret(parcel_eh: EntryHash) -> ExternResult<EntryHash> {
   let notices = DeliveryNotice::query(DeliveryNoticeQueryField::Parcel(parcel_eh))?;
   if notices.len() != 1 {
      return Err(WasmError::Guest(String::from("No Secret found at given EntryHash")));
   }
   let input = RespondToNoticeInput {
      notice_eh,
      has_accepted: true,
   };
   return respond_to_notice(input)?;
}