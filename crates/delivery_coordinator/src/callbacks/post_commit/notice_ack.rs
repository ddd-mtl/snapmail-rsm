use hdk::prelude::*;
use zome_delivery_types::*;
use crate::*;



///
pub fn post_commit_NoticeAck(entry: Entry, _eh: &EntryHash) -> ExternResult<()> {
    let ack = NoticeAck::try_from(entry)?;
    /// Emit Signal
    let res = emit_signal(&SignalProtocol::NewNoticeAck(ack));
    if let Err(err) = res.clone() {
        error!("Emit signal failed: {}", err);
    } else {
        debug!("Emit signal successful!");
    }
    /// Done
    Ok(())
}