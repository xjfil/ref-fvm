use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::sys;

use super::Context;
use crate::kernel::{Result, SendResult};
use crate::Kernel;

/// Send a message to another actor. The result is placed as a CBOR-encoded
/// receipt in the block registry, and can be retrieved by the returned BlockId.
pub fn send(
    context: Context<'_, impl Kernel>,
    recipient_off: u32,
    recipient_len: u32,
    method: u64,
    params_id: u32,
    value_hi: u64,
    value_lo: u64,
) -> Result<sys::out::send::Send> {
    let recipient: Address = context.memory.read_address(recipient_off, recipient_len)?;
    let value = TokenAmount::from_atto((value_hi as u128) << 64 | value_lo as u128);
    // An execution error here means that something went wrong in the FVM.
    // Actor errors are communicated in the receipt.
    let SendResult {
        block_id,
        block_stat,
        exit_code,
    } = context.kernel.send(&recipient, method, params_id, &value)?;
    Ok(sys::out::send::Send {
        exit_code: exit_code.value(),
        return_id: block_id,
        return_codec: block_stat.codec,
        return_size: block_stat.size,
    })
}
