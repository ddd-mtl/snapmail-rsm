use hdk::prelude::*;
use crate::*;

///
#[hdk_extern]
fn recv_remote_signal(dm_signal: SerializedBytes) -> ExternResult<()> {
   debug!("START - {}", dm_signal.bytes().len());
   let dm: DirectMessage = DirectMessage::try_from(dm_signal)
      .map_err(|e| wasm_error!(WasmErrorInner::Serialize(e)))?;
   debug!("dm: {:?}", dm);
   let response_dm = receive_delivery_dm(dm.clone())?;
   debug!("response_dm: {:?}", response_dm);
   /// Send response. It is expected for the Signal sender to still be online.
   let response_response = send_dm(dm.from.clone(), response_dm)?;
   debug!("response_response: {:?}", response_response);
   /// Done
   Ok(())
}