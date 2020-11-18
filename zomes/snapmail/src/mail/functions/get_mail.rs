use hdk3::prelude::*;

use crate::{
    utils::*,
    mail::entries::{
        InMail, OutMail,
    },
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct GetMailOutput(Option<Result<InMail, OutMail>>);


/// Zome Function
/// Get InMail or OutMail struct in local source chain at address
#[hdk_extern]
pub fn get_mail(address: HeaderHash) -> ExternResult<GetMailOutput>{
    /// Get Element at address
    let element = match get!(address)? {
        Some(element) => element,
        None => return Ok(GetMailOutput(None)),
    };
    /// Check if it is an InMail
    let maybe_InMail: ExternResult<InMail> = try_from_element(element.clone());
    if let Ok(inmail) = maybe_InMail {
        return Ok(GetMailOutput(Some(Ok(inmail))));
    }
    /// Check if it is an OutMail
    let maybe_OutMail: ExternResult<OutMail> = try_from_element(element);
    if let Ok(outmail) = maybe_OutMail {
        return Ok(GetMailOutput(Some(Err(outmail))));
    }
    /// Something is wrong...
    debug!("get_mail(): Error. Item found but it is not a Mail!");
    Ok(GetMailOutput(None))
}