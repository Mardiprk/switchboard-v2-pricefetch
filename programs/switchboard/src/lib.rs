use anchor_lang::prelude::*;
use pyth_sdk_solana::state::SolanaPriceAccount;

declare_id!("5NMAuN2UdTN5Qm1RYwUJ6HEGhzGSDeAca1TM2bFcKdqH");

#[program]
pub mod switchboard {

    use super::*;

    pub fn get_prices(ctx: Context<GetPrices>) -> Result<()>{
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        let btc_price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.btc_feed)
            .map_err(|_| error!(ErrorCode::StaleFeed))?;
        let btc_price = btc_price_feed.get_price_unchecked();

        msg!("BTC/USD Price: ${}", btc_price.price);
        msg!("BTC/USD Confidence: ±${}", btc_price.conf);
        msg!("BTC/USD Exponent: ${}", btc_price.expo);

        let sol_price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.sol_feed)
            .map_err(|_| error!(ErrorCode::StaleFeed))?;
        let sol_price = sol_price_feed.get_price_unchecked();

        msg!("SOL/USD Price: ${}", sol_price.price);
        msg!("SOL/USD Confidence: ±${}", sol_price.conf);
        msg!("SOL/USD Exponent: ${}", sol_price.expo);


        let eth_price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.eth_feed)
            .map_err(|_|error!(ErrorCode::StaleFeed))?;
        let eth_price = eth_price_feed.get_price_unchecked();

        msg!("ETH/USD Price: ${}", eth_price.price);
        msg!("ETH/USD Confidence: ±${}", eth_price.conf);
        msg!("ETH/USD Exponent: ${}", eth_price.expo);


        Ok(())
    }
}

#[derive(Accounts)]
pub struct GetPrices<'info>{
    /// CHECK: BTC price account verified off-chain or by address
    pub btc_feed: AccountInfo<'info>,

    /// CHECK: SOL price account verified off-chain or by address
    pub sol_feed: AccountInfo<'info>,
    
    /// CHECK: ETH price account verified off-chain or by address
    pub eth_feed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode{
    #[msg("Price Feed Is Stale")]
    StaleFeed,
}