
mod errors;
mod instructions;
mod msg_codec;
mod state;

use msg_codec::{msg_type, src_eid};
use crate::instructions::quote::Quote;
use anchor_lang::prelude::*;
use errors::*;
use instructions::*;
// use msg_codec::*;
use state::*;
#[allow(unused_imports)]
use oapp::{
    endpoint::{MessagingFee, ID as ENDPOINT_ID},
    endpoint_cpi::LzAccount,
    /*ComposeParams,*/LzReceiveParams,
};



const LZ_RECEIVE_TYPES_SEED: &[u8] = b"LzReceiveTypes";
// const LZ_COMPOSE_TYPES_SEED: &[u8] = b"LzComposeTypes";
const COUNT_SEED: &[u8] = b"Count";
const REMOTE_SEED: &[u8] = b"Remote";

declare_id!("7BSfbpNXwBWyaKErQk5ReWm2zKVcxWjiXQRW2RRJmARf");  // Replace with your actual program ID

#[program]
pub mod lzreceiver {
    use super::*;

    /// Sets the remote sender address for LayerZero verification.
    pub fn set_remote(mut ctx: Context<SetRemote>, params: SetRemoteParams) -> Result<()> {
        SetRemote::apply(&mut ctx, &params)
    }

    /// Estimates the messaging fee for sending a LayerZero message.
    pub fn quote(ctx: Context<Quote>, params: QuoteParams) -> Result<MessagingFee> {
        Quote::apply(&ctx, &params)
    }

    // /// Receives a LayerZero message and processes swap instructions.
    // pub fn lz_receive(mut ctx: Context<LzReceive>, params: LzReceiveParams) -> Result<()> {
    //     LzReceive::apply(&mut ctx, &params)
    // }
    
    pub fn lz_receive(mut ctx: Context<LzReceive>, params: LzReceiveParams) -> Result<()> {
        // let message = params.payload.as_slice();
        let message = params.message.as_slice();
        let message_type = msg_type(message);
        let source_eid = src_eid(message);
    
        msg!("Received message type: {}", message_type);
        msg!("Source EID: {}", source_eid);
    
        LzReceive::apply(&mut ctx, &params)
    }
    /// Returns the required accounts for the `LzReceive` instruction.
    pub fn lz_receive_types(
        ctx: Context<LzReceiveTypes>,
        params: LzReceiveParams,
    ) -> Result<Vec<LzAccount>> {
        LzReceiveTypes::apply(&ctx, &params)
    }
}
























// mod errors;
// mod instructions;
// mod msg_codec;
// mod state;

// use anchor_lang::prelude::*;
// use errors::*;
// use instructions::*;
// use oapp::{endpoint::MessagingFee, endpoint_cpi::LzAccount, LzComposeParams, LzReceiveParams};
// use state::*;

// const LZ_RECEIVE_TYPES_SEED: &[u8] = b"LzReceiveTypes";
// const LZ_COMPOSE_TYPES_SEED: &[u8] = b"LzComposeTypes";
// const COUNT_SEED: &[u8] = b"Count";
// const REMOTE_SEED: &[u8] = b"Remote";

// declare_id!("7BSfbpNXwBWyaKErQk5ReWm2zKVcxWjiXQRW2RRJmARf");

// #[program]
// pub mod lzreceiver {
//     use super::*;

//     pub fn set_remote(mut ctx: Context<SetRemote>, params: SetRemoteParams) -> Result<()> {
//         SetRemote::apply(&mut ctx, &params)
//     }

//     pub fn quote(ctx: Context<Quote>, params: QuoteParams) -> Result<MessagingFee> {
//         Quote::apply(&ctx, &params)
//     }

//     pub fn lz_receive(mut ctx: Context<LzReceive>, params: LzReceiveParams) -> Result<()> {
//         LzReceive::apply(&mut ctx, &params)
//     }

//     pub fn lz_receive_types(
//         ctx: Context<LzReceiveTypes>,
//         params: LzReceiveParams,
//     ) -> Result<Vec<LzAccount>> {
//         LzReceiveTypes::apply(&ctx, &params)
//     }


// #[derive(Accounts)]
// pub struct Initialize {}
