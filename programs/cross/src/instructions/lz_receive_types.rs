use crate::*;
use oapp::endpoint_cpi::{get_accounts_for_clear, get_accounts_for_send_compose, LzAccount};
use oapp::{endpoint::ID as ENDPOINT_ID, LzReceiveParams};

#[derive(Accounts)]
pub struct LzReceiveTypes<'info> {
    #[account(seeds = [COUNT_SEED, &count.id.to_be_bytes()], bump = count.bump)]
    pub count: Account<'info, Count>,
}

impl LzReceiveTypes<'_> {
    /// Determines the accounts required for `LzReceive`, ensuring the right data is provided.
    pub fn apply(
        ctx: &Context<LzReceiveTypes>,
        params: &LzReceiveParams,
    ) -> Result<Vec<LzAccount>> {
        let count = ctx.accounts.count.key();

        // Determine the remote account PDA using the sender's EID
        let seeds = [REMOTE_SEED, &count.to_bytes(), &params.src_eid.to_be_bytes()];
        let (remote, _) = Pubkey::find_program_address(&seeds, ctx.program_id);

        // Define initial required accounts
        let mut accounts = vec![
            LzAccount { pubkey: count, is_signer: false, is_writable: true },  // Count account
            LzAccount { pubkey: remote, is_signer: false, is_writable: false }, // Remote account
        ];

        // Append the required accounts for clearing messages
        let accounts_for_clear = get_accounts_for_clear(
            ENDPOINT_ID,
            &count,
            params.src_eid,
            &params.sender,
            params.nonce,
        );
        accounts.extend(accounts_for_clear);

        // If the message type is composed, add required accounts for composing messages
        let is_composed = msg_codec::msg_type(&params.message) == msg_codec::COMPOSED_TYPE;
        if is_composed {
            let accounts_for_composing = get_accounts_for_send_compose(
                ENDPOINT_ID,
                &count,
                &count, // self
                &params.guid,
                0,
                &params.message,
            );
            accounts.extend(accounts_for_composing);
        }

        Ok(accounts)
    }
}
