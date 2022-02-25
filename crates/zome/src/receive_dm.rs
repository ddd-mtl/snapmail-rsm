use hdk::prelude::*;
use crate::utils::*;

use zome_delivery_types::*;
use crate::functions::*;
use crate::dm_protocol::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectMessage {
    pub from: AgentPubKey,
    pub msg: DeliveryProtocol,
}

/// Start point for any remote call in this zome
/// WARN: Name of function must match REMOTE_ENDPOINT const value
#[hdk_extern]
pub fn receive_delivery_dm(dm: DirectMessage) -> ExternResult<DeliveryProtocol> {
    debug!("Received DM: from: {} | msg: {}", dm.from, dm.msg);
    let reply = match dm.msg {
        DeliveryProtocol::ChunkRequest(_chunk_eh) => { /* FIXME */ DeliveryProtocol::Pong },
        DeliveryProtocol::ParcelRequest(distribution_eh) => {
            receive_dm_parcel_request(dm.from, distribution_eh)
        },
        DeliveryProtocol::Item(item) => {
            match item.kind {
                ItemKind::DeliveryNotice => receive_dm_notice(dm.from, item),
                ItemKind::DeliveryReply  => receive_dm_reply(dm.from, item),
                //ItemKind::Entry => {/* FIXME */},
                ItemKind::ParcelReceived => receive_dm_reception(dm.from, item),
                _ => panic!("FIXME kind not supported yet"),
            }
        },
        DeliveryProtocol::Ping => DeliveryProtocol::Pong,
        _ => {
             DeliveryProtocol::Failure("Unexpected protocol".to_owned())
        },
    };
    Ok(reply)
}


/// Returns Success or Failure
pub fn receive_dm_parcel_request(from: AgentPubKey, distribution_eh: EntryHash) -> DeliveryProtocol {
    /// Get Distribution Entry
    let maybe_distribution: ExternResult<Distribution> = get_typed_from_eh(distribution_eh);
    if let Err(err) = maybe_distribution {
        return failure_err("Distribution not found", err);
    }
    let distribution = maybe_distribution.unwrap();
    /// Make sure DM is from a valid recipient
    if !distribution.recipients.contains(&from) {
        return failure("Request from invalid recipient");
    }
    /// Get Parcel entry hash
    let parcel_eh = distribution.parcel_summary.reference.entry_address();
    debug!("Looking for Parcel: {:?}", parcel_eh);
    /// Get entry
    let maybe_maybe_element = get(parcel_eh, GetOptions::content());
    if let Err(err) = maybe_maybe_element {
        return failure_err("Failed to get Parcel Element", err);
    }
    let element = match maybe_maybe_element.unwrap() {
        None => return failure("Parcel Element not found"),
        Some(el) => el,
    };
    /// Return Entry
    //debug!("Parcel Element found: {:?}", element);
    let maybe_entry = element.entry().as_option();
    if maybe_entry.is_none() {
        return failure("Parcel Entry not found in Parcel Element");
    }
    return DeliveryProtocol::ParcelResponse(maybe_entry.unwrap().to_owned());
}


/// Commit received DeliveryNotice from sender
/// Returns Success or Failure
pub fn receive_dm_notice(from: AgentPubKey, item: PendingItem) -> DeliveryProtocol {
    let maybe_maybe_notice: ExternResult<Option<DeliveryNotice>> = unpack_item(item, from.clone());
    if let Err(err) = maybe_maybe_notice {
        return failure_err("Failed deserializing DeliveryNotice", err);
    }
    let maybe_notice = maybe_maybe_notice.unwrap();
    if maybe_notice.is_none() {
        return failure("Failed deserializing DeliveryNotice (2)");
    }
    /// Commit DeliveryNotice
    let maybe_hh = create_entry(&maybe_notice.unwrap());
    if let Err(err) = maybe_hh {
        return failure_err("Failed committing DeliveryNotice", err);
    }
    /// Return Success
    return DeliveryProtocol::Success;
}

/// Create and commit a ReplyReceived from a DeliveryReply
/// Returns Success or Failure
pub fn receive_dm_reply(from: AgentPubKey, pending_item: PendingItem) -> DeliveryProtocol {
    let maybe_maybe_reply: ExternResult<Option<DeliveryReply>> = unpack_item(pending_item.clone(), from.clone());
    if let Err(err) = maybe_maybe_reply {
        return failure_err("Failed deserializing DeliveryReply", err);
    }
    let maybe_reply = maybe_maybe_reply.unwrap();
    if maybe_reply.is_none() {
        return failure("Failed deserializing DeliveryReply (2)");
    }
    /// Create ReplyReceived
    let receipt = ReplyReceived {
        distribution_eh: pending_item.distribution_eh,
        recipient: from,
        recipient_signature: pending_item.author_signature,
        has_accepted: maybe_reply.unwrap().has_accepted,
        //date: now(),
    };
    /// Commit ReplyReceived
    let maybe_hh = create_entry(&receipt);
    if let Err(err) = maybe_hh {
        return failure_err("Failed committing ReplyReceived", err);
    }
    /// Return Success
    return DeliveryProtocol::Success;
}


/// Returns Success or Failure
pub fn receive_dm_reception(from: AgentPubKey, pending_item: PendingItem) -> DeliveryProtocol {
    let maybe_received: ExternResult<Option<ParcelReceived>> = unpack_item(pending_item.clone(),from.clone());
    if let Err(err) = maybe_received {
        return failure_err("Failed deserializing ParcelReceived", err);
    }
    /// Create DeliveryReceipt
    let receipt = DeliveryReceipt {
        distribution_eh: pending_item.distribution_eh,
        recipient: from,
        recipient_signature: pending_item.author_signature,
        //date_of_response: now(),
    };
    /// Commit DeliveryReceipt
    let maybe_hh = create_entry(&receipt);
    if let Err(err) = maybe_hh {
        let response_str = "Failed committing DeliveryReceipt";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Return Success
    return DeliveryProtocol::Success;
}