use anchor_lang::prelude::*;
use pyth_sdk_solana::state::SolanaPriceAccount;

declare_id!("5NMAuN2UdTN5Qm1RYwUJ6HEGhzGSDeAca1TM2bFcKdqH");

#[program]
pub mod switchboard {
    use super::*;

    pub fn get_prices(ctx: Context<GetPrices>) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        msg!("Current timestamp: {}", current_timestamp);

        // ----- BTC -----
        let btc_price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.btc_feed)
            .map_err(|e| {
                msg!("Error loading BTC feed: {:?}", e);
                error!(ErrorCode::InvalidPriceFeed)
            })?;
        
        // Use get_price_unchecked for devnet (feeds may not update frequently)
        let btc_price = btc_price_feed.get_price_unchecked();
        let btc_usd = (btc_price.price as f64) * 10_f64.powi(btc_price.expo);
        let btc_age = btc_price.publish_time;

        msg!("BTC/USD: ${:.3}", btc_usd);
        msg!("Last Updated: {} seconds ago ({} hours)", btc_age, btc_age / 3600);

        // ----- SOL -----
        let sol_price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.sol_feed)
            .map_err(|e| {
                msg!("Error loading SOL feed: {:?}", e);
                error!(ErrorCode::InvalidPriceFeed)
            })?;
        
        let sol_price = sol_price_feed.get_price_unchecked();
        let sol_usd = (sol_price.price as f64) * 10_f64.powi(sol_price.expo);
        let sol_age = sol_price.publish_time;

        msg!("SOL/USD: ${:.3}", sol_usd);
        msg!("Last Updated: {} seconds ago ({} hours)", sol_age, sol_age / 3600);
        
        // ----- ETH -----
        let eth_price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.eth_feed)
            .map_err(|e| {
                msg!("Error loading ETH feed: {:?}", e);
                error!(ErrorCode::InvalidPriceFeed)
            })?;
        
        let eth_price = eth_price_feed.get_price_unchecked();
        let eth_usd = (eth_price.price as f64) * 10_f64.powi(eth_price.expo);
        let eth_age = eth_price.publish_time;

        msg!("ETH/USD: ${:.3}", eth_usd);
        msg!("Last Updated: {} seconds ago ({} hours)", eth_age, eth_age / 3600);
        
        Ok(())
    }

    // Production version with staleness check (use on mainnet)
    pub fn get_prices_with_validation(ctx: Context<GetPrices>) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        let max_age = 60; 

        msg!("Current Timestamp: {}", current_timestamp);

        // ----- BTC -----
        let btc_price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.btc_feed).map_err(|_| error!(ErrorCode::InvalidPriceFeed))?;
        let btc_price = btc_price_feed.get_price_no_older_than(current_timestamp, max_age).ok_or_else(|| {
            msg!("BTC price is stale. Max age: {}s", max_age);
            error!(ErrorCode::StaleFeed)
        })?;

        let btc_normalized = (btc_price.price as f64) * 10_f64.powi(btc_price.expo);

        msg!("BTC/USD: ${:.2}", btc_normalized);

        // ----- SOL -----
        let sol_price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.sol_feed).map_err(|_| error!(ErrorCode::InvalidPriceFeed))?;
        let sol_price = sol_price_feed.get_price_no_older_than(current_timestamp, max_age).ok_or_else(|| {
            msg!("SOL price is stale. Max age: {}s", max_age);
            error!(ErrorCode::StaleFeed)
        })?;
        let sol_normalized = (sol_price.price as f64) * 10_f64.powi(sol_price.expo);

        msg!("SOL/USD {:.2}", sol_normalized);
        
        // ----- ETH -----
        let eth_price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.eth_feed).map_err(|_| error!(ErrorCode::InvalidPriceFeed))?;
        let eth_price = eth_price_feed.get_price_no_older_than(current_timestamp, max_age).ok_or_else(||{
            msg!("ETH price is stale, Max age: {}s", max_age);
            error!(ErrorCode::StaleFeed)
        })?;
        
        let eth_normalized = (eth_price.price as f64) * 10_f64.powi(eth_price.expo);
        
        msg!("ETH/USD ${:.2}", eth_normalized);

        Ok(())
    }

}

#[derive(Accounts)]
pub struct GetPrices<'info> {
    /// CHECK: BTC price account verified by Pyth SDK
    pub btc_feed: AccountInfo<'info>,

    /// CHECK: SOL price account verified by Pyth SDK
    pub sol_feed: AccountInfo<'info>,

    /// CHECK: ETH price account verified by Pyth SDK
    pub eth_feed: AccountInfo<'info>,

    
}

#[error_code]
pub enum ErrorCode {
    #[msg("Price Feed Is Stale")]
    StaleFeed,
    #[msg("Invalid Price Feed Account")]
    InvalidPriceFeed,
}