use hdk::prelude::*;

/// Entry for confirming a manifest has been well received by a recipient
#[hdk_entry(id = "NoticeReceived", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct NoticeReceived {
    pub distribution_eh: EntryHash,
    pub recipient: AgentPubKey,
    pub recipient_manifest_signature: Signature,
    pub date_of_reception: u64,
}