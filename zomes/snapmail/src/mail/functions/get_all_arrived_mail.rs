use hdk3::prelude::*;
use hdk3::prelude::query::ChainQueryFilter;

use crate::{
    ZomeHeaderHashVec,
    link_kind, entry_kind,
    def_to_type,
    utils::*,
};

/// Zome Function
/// Return list of all InMails that this agent did not acknowledge.
#[hdk_extern]
pub fn get_all_arrived_mail(_: ()) -> ExternResult<ZomeHeaderHashVec> {
    /// 1. Get all InMails with query
    let inmail_query_args = ChainQueryFilter::default()
       .include_entries(true)
       .entry_type(def_to_type(entry_kind::InMail));
    let maybe_inmail_result = query!(inmail_query_args);
    if let Err(err) = maybe_inmail_result {
        debug!(format!("get_all_mails() inmail_result failed: {:?}", err)).ok();
        return Err(hdk3::error::HdkError::SerializedBytes(err));
        //return Err(err);
    }
    let inmails: Vec<Element> = maybe_inmail_result.unwrap().0;
    debug!(format!(" get_all_arrived_mail() inmails: {:?}", inmails)).ok();

    // DEBUG - Output dummy values instead
    // let mut unreads = Vec::new();
    // let dummy: HashString = HashString::from("QmYgC6qzGYDZyfp5xcyMH58cnBRde29Ent4JshSk629Qz6");
    // for _ in  0..2000 {
    //     unreads.push(dummy.clone());
    // }

    /// For each InMail
    let mut unreads = Vec::new();
    for inmail in inmails {
        let header_address = inmail.header_hashed().as_hash().to_owned();
        let header = inmail.header();
        let inmail_address = header.entry_hash().expect("Should have an Entry");

        /// 2. Get Acknowledgment private link
        // let res = hdk::get_links_count(
        //     inmail_address,
        //     LinkMatch::Exactly(link_kind::Acknowledgment),
        //     LinkMatch::Any,
        // )?;
        // if res.count > 0 {
        //     continue;
        // }
        let links_result = get_links!(inmail_address.clone(), link_tag(link_kind::Acknowledgment))
           ?.into_inner();
        /// If link found, it means Ack has not been received
        if links_result.len() > 0 {
            continue;
        }

        /// Add to result list
        unreads.push(header_address.clone());
    }
    Ok(ZomeHeaderHashVec(unreads))
}