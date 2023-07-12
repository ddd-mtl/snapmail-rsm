use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;

///
pub fn get_all_inbox_items(maybe_kind: Option<ItemKind>) -> ExternResult<Vec<(PendingItem, Link)>> {
    /// Get typed targets
    let my_agent_eh = EntryHash::from(agent_info()?.agent_latest_pubkey);
    let mut pending_pairs = get_typed_from_links::<PendingItem>(
        my_agent_eh.clone(),
        LinkTypes::Inbox,
        None,
    )?;
    /// Filter
    if maybe_kind.is_some() {
        let kind = maybe_kind.unwrap();
        pending_pairs.retain(|pair|  pair.0.kind == kind)
    }
    /// Done
    Ok(pending_pairs)
}


/// Call The Zome owner of the entry to commit it
pub fn call_commit_parcel(entry: Entry, notice: &DeliveryNotice, maybe_link_ah: Option<ActionHash>)
    -> ExternResult<ActionHash>
{
    let input = CommitParcelInput {
        zome_index: ZomeIndex::from(get_zome_index(notice.summary.parcel_reference.entry_zome_name())?),
        entry_index: notice.summary.parcel_reference.entry_index(),
        entry_visibility: notice.summary.parcel_reference.entry_visibility(),
        entry: entry.clone(),
        maybe_link_ah: maybe_link_ah.clone(),
    };

    /// Make sure CreateLink exists
    if let Some(link_hh) = maybe_link_ah {
        let maybe_el = get(link_hh.clone(), GetOptions::default())?;
        if maybe_el.is_none() {
            return zome_error!("call_commit_parcel(): CreateLink not found.");
        }
    }

    debug!("call_commit_parcel() zome_name = {:?}", notice.summary.parcel_reference.entry_zome_name());
    let response = call_remote(
        agent_info()?.agent_latest_pubkey,
        notice.summary.parcel_reference.entry_zome_name(),
        COMMIT_PARCEL_CALLBACK_NAME.into(),
        None,
        input.clone(),
    )?;
    let ah = decode_response(response)?;

    // /// Delete Link
    // if let Some(link_hh) = input.maybe_link_hh {
    //    debug!("call_commit_parcel() delete_link {:?}", link_hh);
    //        let input = DeleteLinkInput::new(link_hh,
    //           ChainTopOrdering::Relaxed,
    //        );
    //     let _hh = HDK.with(|h| {
    //         h.borrow()
    //          .delete_link(input)
    //     })?;
    // }

    /// Create ParcelReceived if its an AppEntry
    /// (for a Manifest, we have to wait for all chunks to be received)
    if let ParcelReference::AppEntry(..) = notice.summary.parcel_reference {
        let received = ParcelReceived {
            notice_eh: hash_entry(notice.clone())?,
            parcel_eh: hash_entry(entry.clone())?,
        };
        let response = call_self("commit_ParcelReceived", received.clone())?;
        let received_eh: EntryHash = decode_response(response)?;
        debug!("call_commit_parcel() received_eh = {:?}", received_eh);
        /// Emit Signal
        let res = emit_signal(&SignalProtocol::ReceivedParcel(received));
        if let Err(err) = res {
            error!("Emit signal failed: {}", err);
        }
    }
    /// Done
    Ok(ah)
}


/// Return size of an AppEntry
pub fn get_app_entry_size(eh: EntryHash) -> ExternResult<usize> {
    /// Get Element
    let maybe_element = get(eh, GetOptions::content())?;
    let element = match maybe_element {
        Some(el) => el,
        None => return error("No element found at given payload address"),
    };
    /// Get length of SerializedBytes
    let entry = element
       .entry()
       .as_option()
       .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("No AppEntry found at given payload address"))))
       ?
       .to_owned();
    if let Entry::App(app_bytes) = entry {
        let size: usize = app_bytes
           .into_sb()
           .bytes()
           .len();
        /// Done
        return Ok(size);
    }
    error("Entry not of type App()")
}


// ///
// pub fn sign_parcel(parcel: &Parcel) -> ExternResult<Signature> {
//     let me = agent_info()?.agent_latest_pubkey;
//     let signature = sign(me, parcel)?;
//     Ok(signature)
// }