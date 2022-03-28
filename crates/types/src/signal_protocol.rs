use hdk::prelude::*;

use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalKind {
    ReceivedNotice,
    ReceivedReply,
    ReceivedParcel,
    ReceivedReceipt,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    ReceivedNotice(DeliveryNotice),
    ReceivedReply(ReplyReceived),
    ReceivedParcel(ParcelReceived),
    ReceivedReceipt(DeliveryReceipt),
}

impl SignalProtocol {
    pub fn is(&self, kind: &SignalKind, eh: &EntryHash) -> bool {
        match kind {
            SignalKind::ReceivedNotice => {
                if let SignalProtocol::ReceivedNotice(notice) = self {
                    return &notice.distribution_eh == eh;
                }
                false
            },
            SignalKind::ReceivedReply => {
                if let SignalProtocol::ReceivedReply(entry) = self {
                    return &entry.distribution_eh == eh;
                }
                false
            },
            SignalKind::ReceivedParcel => {
                if let SignalProtocol::ReceivedParcel(received) = self {
                    return &received.parcel_eh == eh;
                }
                false
            },
            SignalKind::ReceivedReceipt => {
                if let SignalProtocol::ReceivedReceipt(entry) = self {
                    return &entry.distribution_eh == eh;
                }
                false
            },
        }
    }
}