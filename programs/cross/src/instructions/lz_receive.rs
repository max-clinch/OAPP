use crate::*;
use anchor_lang::prelude::*;
use bincode::{config, decode_from_slice, Encode, Decode}; // ✅ Import bincode traits
use oapp::{
    endpoint::{
        cpi::accounts::Clear,
        instructions::{ClearParams, SendComposeParams},
        ConstructCPIContext, ID as ENDPOINT_ID,
    },
    LzReceiveParams,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Encode, Decode)] 
pub struct SwapParams {
    pub token_in: [u8; 32],  
    pub token_out: [u8; 32], 
    pub amount_in: u64,
    pub min_amount_out: u64,
    pub path: Vec<[u8; 32]>,  
    pub dex_choice: u8,
    pub deadline: u64,
    pub dex_address: [u8; 32],  
    pub recipient: [u8; 32],    
    pub fee: u32,
    pub sqrt_price_limit_x96: u128,
}

#[derive(Accounts)]
#[instruction(params: LzReceiveParams)]
pub struct LzReceive<'info> {
    #[account(mut, seeds = [COUNT_SEED, &count.id.to_be_bytes()], bump = count.bump)]
    pub count: Account<'info, Count>,
    
    #[account(
        seeds = [REMOTE_SEED, &count.key().to_bytes(), &params.src_eid.to_be_bytes()],
        bump = remote.bump,
        constraint = params.sender == remote.address
    )]
    pub remote: Account<'info, Remote>,
}

impl LzReceive<'_> {
    pub fn apply(ctx: &mut Context<LzReceive>, params: &LzReceiveParams) -> Result<()> {
        let seeds: &[&[u8]] =
            &[COUNT_SEED, &ctx.accounts.count.id.to_be_bytes(), &[ctx.accounts.count.bump]];

        // Decode SwapParams from message
        let swap_params: SwapParams = match decode_from_slice(&params.message, config::standard()) {
            Ok((decoded, _)) => decoded, 
            Err(_) => return Err(CounterError::InvalidMessageType.into()),
        };

        // Convert `[u8; 32]` back to `Pubkey`
        let token_in_pubkey = Pubkey::new_from_array(swap_params.token_in);
        let recipient_pubkey = Pubkey::new_from_array(swap_params.recipient);

        let expected_recipient_pubkey = ctx.accounts.count.key();

        // Verify sender
        require!(
            params.sender == ctx.accounts.remote.address,
            CounterError::UnauthorizedSender
        );
        require!(
            recipient_pubkey == expected_recipient_pubkey, 
            CounterError::UnauthorizedRecipient
        );

        // Verify recipient
require!(
    recipient_pubkey == expected_recipient_pubkey, 
    CounterError::UnauthorizedRecipient
);

        

        // Clear the message first
        let accounts_for_clear = &ctx.remaining_accounts[0..Clear::MIN_ACCOUNTS_LEN];
        oapp::endpoint_cpi::clear(
            ENDPOINT_ID,
            ctx.accounts.count.key(),
            accounts_for_clear,
            seeds,
            ClearParams {
                receiver: ctx.accounts.count.key(),
                src_eid: params.src_eid,
                sender: params.sender,
                nonce: params.nonce,
                guid: params.guid,
                message: params.message.clone(),
            },
        ).map_err(|_| CounterError::ClearFailed)?;

        // Process message type
        let msg_type = msg_codec::msg_type(&params.message);
        match msg_type {
            msg_codec::VANILLA_TYPE => {
                ctx.accounts.count.count += 1;
                msg!("Received Vanilla Swap Message!");
            }
            msg_codec::COMPOSED_TYPE => {
                ctx.accounts.count.count += 1;
                msg!("Received Composed Swap Message! Sending response...");

                oapp::endpoint_cpi::send_compose(
                    ENDPOINT_ID,
                    ctx.accounts.count.key(),
                    &ctx.remaining_accounts[Clear::MIN_ACCOUNTS_LEN..],
                    seeds,
                    SendComposeParams {
                        to: ctx.accounts.count.key(), // self
                        guid: params.guid,
                        index: 0,
                        message: params.message.clone(),
                    },
                ).map_err(|_| CounterError::SendComposeFailed)?;
            }
            _ => return Err(CounterError::InvalidMessageType.into()),
        }

        msg!(
            " Received Swap Message: TokenIn: {:?}, TokenOut: {:?}, AmountIn: {:?}",
            token_in_pubkey,
            Pubkey::new_from_array(swap_params.token_out),
            swap_params.amount_in
        );

        Ok(())
    }
}





























// // use crate::*;
// // use anchor_lang::prelude::*;
// // use oapp::{
// //     endpoint::{
// //         cpi::accounts::Clear,
// //         instructions::{ClearParams, SendComposeParams},
// //         ConstructCPIContext, ID as ENDPOINT_ID,
// //     },
// //     LzReceiveParams,
// // };

// // #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
// // pub struct SwapParams {
// //     pub token_in: Pubkey,
// //     pub token_out: Pubkey,
// //     pub amount_in: u64,
// //     pub min_amount_out: u64,
// //     pub path: Vec<Pubkey>,
// //     pub dex_choice: u8,
// //     pub deadline: u64,
// //     pub dex_address: Pubkey,
// //     pub recipient: Pubkey,
// //     pub fee: u32,
// //     pub sqrt_price_limit_x96: u128,
// // }

// // #[derive(Accounts)]
// // #[instruction(params: LzReceiveParams)]
// // pub struct LzReceive<'info> {
// //     #[account(mut, seeds = [COUNT_SEED, &count.id.to_be_bytes()], bump = count.bump)]
// //     pub count: Account<'info, Count>,
// //     #[account(
// //         seeds = [REMOTE_SEED, &count.key().to_bytes(), &params.src_eid.to_be_bytes()],
// //         bump = remote.bump,
// //         constraint = params.sender == remote.address
// //     )]
// //     pub remote: Account<'info, Remote>,
// // }

// // impl LzReceive<'_> {
// //     pub fn apply(ctx: &mut Context<LzReceive>, params: &LzReceiveParams) -> Result<()> {
// //         let seeds: &[&[u8]] =
// //             &[COUNT_SEED, &ctx.accounts.count.id.to_be_bytes(), &[ctx.accounts.count.bump]];

// //         // Decode SwapParams from message
// //         let swap_params: SwapParams = match bincode::deserialize(&params.message) {
// //             Ok(decoded) => decoded,
// //             Err(_) => return Err(CounterError::InvalidMessageType.into()),
// //         };

// //         // Verify sender
// //         require!(
// //             params.sender == ctx.accounts.remote.address,
// //             CounterError::UnauthorizedSender
// //         );

// //         // Clear the message first
// //         let accounts_for_clear = &ctx.remaining_accounts[0..Clear::MIN_ACCOUNTS_LEN];
// //         let _ = oapp::endpoint_cpi::clear(
// //             ENDPOINT_ID,
// //             ctx.accounts.count.key(),
// //             accounts_for_clear,
// //             seeds,
// //             ClearParams {
// //                 receiver: ctx.accounts.count.key(),
// //                 src_eid: params.src_eid,
// //                 sender: params.sender,
// //                 nonce: params.nonce,
// //                 guid: params.guid,
// //                 message: params.message.clone(),
// //             },
// //         )?;

// //         // Process message type
// //         let msg_type = msg_codec::msg_type(&params.message);
// //         match msg_type {
// //             msg_codec::VANILLA_TYPE => {
// //                 ctx.accounts.count.count += 1;
// //                 msg!("Received Vanilla Swap Message!");
// //             }
// //             msg_codec::COMPOSED_TYPE => {
// //                 ctx.accounts.count.count += 1;
// //                 msg!("Received Composed Swap Message! Sending response...");

// //                 oapp::endpoint_cpi::send_compose(
// //                     ENDPOINT_ID,
// //                     ctx.accounts.count.key(),
// //                     &ctx.remaining_accounts[Clear::MIN_ACCOUNTS_LEN..],
// //                     seeds,
// //                     SendComposeParams {
// //                         to: ctx.accounts.count.key(), // self
// //                         guid: params.guid,
// //                         index: 0,
// //                         message: params.message.clone(),
// //                     },
// //                 )?;
// //             }
// //             _ => return Err(CounterError::InvalidMessageType.into()),
// //         }

// //         msg!(
// //             "Received Swap Message: TokenIn: {:?}, TokenOut: {:?}, AmountIn: {:?}",
// //             swap_params.token_in,
// //             swap_params.token_out,
// //             swap_params.amount_in
// //         );

// //         Ok(())
// //     }
// // }



// use crate::*;
// use bincode::config::Configuration; // Import the configuration
// use bincode::decode_from_slice; 
// use anchor_lang::prelude::*;
// use bincode::{config, Decode, Encode}; // ✅ Ensure `Decode` is imported

// use oapp::{
//     endpoint::{
//         cpi::accounts::Clear,
//         instructions::{ClearParams, SendComposeParams},
//         ConstructCPIContext, ID as ENDPOINT_ID,
//     },
//     LzReceiveParams,
// };

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Encode, Decode)]
// pub struct SwapParams {
//     pub token_in: [u8; 32],  // ✅ Store as `[u8; 32]` instead of `Pubkey`
//     pub token_out: [u8; 32], // ✅ Fixes `Encode` trait error
//     pub amount_in: u64,
//     pub min_amount_out: u64,
//     pub path: Vec<[u8; 32]>,  // ✅ Convert Vec<Pubkey> → Vec<[u8; 32]>
//     pub dex_choice: u8,
//     pub deadline: u64,
//     pub dex_address: [u8; 32],  // ✅ Convert `Pubkey` to `[u8; 32]`
//     pub recipient: [u8; 32],    // ✅ Convert `Pubkey` to `[u8; 32]`
//     pub fee: u32,
//     pub sqrt_price_limit_x96: u128,
// }


// // #[derive(AnchorSerialize, AnchorDeserialize, Clone, Encode, Decode)] // ✅ Add `Encode` and `Decode`
// // pub struct SwapParams {
// //     pub token_in: Pubkey,
// //     pub token_out: Pubkey,
// //     pub amount_in: u64,
// //     pub min_amount_out: u64,
// //     pub path: Vec<Pubkey>,
// //     pub dex_choice: u8,
// //     pub deadline: u64,
// //     pub dex_address: Pubkey,
// //     pub recipient: Pubkey,
// //     pub fee: u32,
// //     pub sqrt_price_limit_x96: u128,
// // }

// #[derive(Accounts)]
// #[instruction(params: LzReceiveParams)]
// pub struct LzReceive<'info> {
//     #[account(mut, seeds = [COUNT_SEED, &count.id.to_be_bytes()], bump = count.bump)]
//     pub count: Account<'info, Count>,
//     #[account(
//         seeds = [REMOTE_SEED, &count.key().to_bytes(), &params.src_eid.to_be_bytes()],
//         bump = remote.bump,
//         constraint = params.sender == remote.address
//     )]
//     pub remote: Account<'info, Remote>,
// }

// impl LzReceive<'_> {
//     pub fn apply(ctx: &mut Context<LzReceive>, params: &LzReceiveParams) -> Result<()> {
//         let seeds: &[&[u8]] =
//             &[COUNT_SEED, &ctx.accounts.count.id.to_be_bytes(), &[ctx.accounts.count.bump]];

//         // // Decode SwapParams from message
//         // let swap_params: SwapParams = match bincode::deserialize(&params.message) {
//         //     Ok(decoded) => decoded,
//         //     Err(_) => return Err(CounterError::InvalidMessageType.into()),
//         // };

//         // Decode SwapParams from message
// // let swap_params: SwapParams = match decode_from_slice(&params.message, bincode::config::standard()) {
// //     Ok((decoded, _)) => decoded, // `decoded` is the deserialized data
// //     Err(_) => return Err(CounterError::InvalidMessageType.into()),
// // };   
// let swap_params = SwapParams {
//     token_in: ctx.accounts.token_in.key().to_bytes(), // ✅ Convert `Pubkey` to `[u8; 32]`
//     token_out: ctx.accounts.token_out.key().to_bytes(),
//     dex_address: ctx.accounts.dex_address.key().to_bytes(),
//     recipient: ctx.accounts.recipient.key().to_bytes(),
//     path: ctx.accounts.path.iter().map(|p| p.key().to_bytes()).collect(),
//     amount_in: 1000,
//     min_amount_out: 900,
//     dex_choice: 1,
//     deadline: 1678900000,
//     fee: 5,
//     sqrt_price_limit_x96: 1234567890,
// };


//         // Verify sender
//         require!(
//             params.sender == ctx.accounts.remote.address,
//             CounterError::UnauthorizedSender
//         );

//         // Clear the message first
//         let accounts_for_clear = &ctx.remaining_accounts[0..Clear::MIN_ACCOUNTS_LEN];
//         oapp::endpoint_cpi::clear(
//             ENDPOINT_ID,
//             ctx.accounts.count.key(),
//             accounts_for_clear,
//             seeds,
//             ClearParams {
//                 receiver: ctx.accounts.count.key(),
//                 src_eid: params.src_eid,
//                 sender: params.sender,
//                 nonce: params.nonce,
//                 guid: params.guid,
//                 message: params.message.clone(),
//             },
//         ).map_err(|_| CounterError::ClearFailed)?;

//         // Process message type
//         let msg_type = msg_codec::msg_type(&params.message);
//         match msg_type {
//             msg_codec::VANILLA_TYPE => {
//                 ctx.accounts.count.count += 1;
//                 msg!("Received Vanilla Swap Message!");
//             }
//             msg_codec::COMPOSED_TYPE => {
//                 ctx.accounts.count.count += 1;
//                 msg!("Received Composed Swap Message! Sending response...");

//                 oapp::endpoint_cpi::send_compose(
//                     ENDPOINT_ID,
//                     ctx.accounts.count.key(),
//                     &ctx.remaining_accounts[Clear::MIN_ACCOUNTS_LEN..],
//                     seeds,
//                     SendComposeParams {
//                         to: ctx.accounts.count.key(), // self
//                         guid: params.guid,
//                         index: 0,
//                         message: params.message.clone(),
//                     },
//                 ).map_err(|_| CounterError::SendComposeFailed)?;
//             }
//             _ => return Err(CounterError::InvalidMessageType.into()),
//         }

//         msg!(
//             "Received Swap Message: TokenIn: {:?}, TokenOut: {:?}, AmountIn: {:?}",
//             swap_params.token_in,
//             swap_params.token_out,
//             swap_params.amount_in
//         );

//         Ok(())
//     }
// }
