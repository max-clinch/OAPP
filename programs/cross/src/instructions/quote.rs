use crate::*;
use anchor_lang::prelude::*;
use oapp::endpoint::{
    instructions::QuoteParams as EndpointQuoteParams, state::EndpointSettings, ENDPOINT_SEED,
    ID as ENDPOINT_ID,
};

#[derive(Accounts)]
#[instruction(params: QuoteParams)]
pub struct Quote<'info> {
    #[account(seeds = [COUNT_SEED, &count.id.to_be_bytes()], bump = count.bump)]
    pub count: Account<'info, Count>,
    #[account(seeds = [ENDPOINT_SEED], bump = endpoint.bump, seeds::program = ENDPOINT_ID)]
    pub endpoint: Account<'info, EndpointSettings>, // Ensure EndpointSettings implements AnchorSerialize/Deserialize
}

impl<'info> Quote<'info> {
    pub fn apply(ctx: &Context<Quote>, params: &QuoteParams) -> Result<MessagingFee> {
        // Encode the message type
        let message = msg_codec::encode(params.msg_type, ctx.accounts.endpoint.eid);

        // Prepare the quote parameters
        let quote_params = EndpointQuoteParams {
            sender: ctx.accounts.count.key(),
            dst_eid: params.dst_eid,
            receiver: params.receiver,
            message,
            pay_in_lz_token: params.pay_in_lz_token,
            options: params.options.clone(),
        };

        // Call LayerZero Endpoint CPI to get the fee estimate
        oapp::endpoint_cpi::quote(ENDPOINT_ID, ctx.remaining_accounts, quote_params)
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteParams {
    pub dst_eid: u32,
    pub receiver: [u8; 32],
    pub msg_type: u8,
    pub options: Vec<u8>,
    pub pay_in_lz_token: bool,
}

























// use crate::*;
// use anchor_lang::prelude::*;
// use oapp::endpoint::{
//     instructions::QuoteParams as EndpointQuoteParams, state::EndpointSettings, ENDPOINT_SEED,
//     ID as ENDPOINT_ID,
// };

// #[derive(Accounts)]
// #[instruction(params: QuoteParams)]
// pub struct Quote<'info> {
//     #[account(seeds = [COUNT_SEED, &count.id.to_be_bytes()], bump = count.bump)]
//     pub count: Account<'info, Count>,
//     #[account(seeds = [ENDPOINT_SEED], bump = endpoint.bump, seeds::program = ENDPOINT_ID)]
//     pub endpoint: Account<'info, EndpointSettings>,
// }

// impl<'info> Quote<'info> {
//     pub fn apply(ctx: &Context<Quote>, params: &QuoteParams) -> Result<MessagingFee> {
//         // Encode the message type
//         let message = msg_codec::encode(params.msg_type, ctx.accounts.endpoint.eid);

//         // Prepare the quote parameters
//         let quote_params = EndpointQuoteParams {
//             sender: ctx.accounts.count.key(),
//             dst_eid: params.dst_eid,
//             receiver: params.receiver,
//             message,
//             pay_in_lz_token: params.pay_in_lz_token,
//             options: params.options.clone(),
//         };

//         // Call LayerZero Endpoint CPI to get the fee estimate
//         oapp::endpoint_cpi::quote(ENDPOINT_ID, ctx.remaining_accounts, quote_params)
//     }
// }

// #[derive(Clone, AnchorSerialize, AnchorDeserialize)]
// pub struct QuoteParams {
//     pub dst_eid: u32,
//     pub receiver: [u8; 32],
//     pub msg_type: u8,
//     pub options: Vec<u8>,
//     pub pay_in_lz_token: bool,
// }

