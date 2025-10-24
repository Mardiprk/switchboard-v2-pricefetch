use anchor_lang::prelude::*;
use switchboard_v2::AggregatorAccountData;
use std::convert::TryInto;

declare_id!("5NMAuN2UdTN5Qm1RYwUJ6HEGhzGSDeAca1TM2bFcKdqH");

#[program]
pub mod switchboard {
    use super::*;

    pub fn get_prices(ctx: Context<GetPrices>) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        msg!("Current timestamp: {}", current_timestamp);

        // ----- BTC -----
        let btc_feed = &mut ctx.accounts.btc_feed;
        let btc_result = parse_switchboard_feed(&btc_feed)?;
        let btc_age = current_timestamp - btc_result.timestamp;

        msg!("BTC/USD ${:.2}", btc_result.price);
        msg!("Last Updated: {} seconds ago ({:.2}) hours", btc_age, btc_age as f64 / 3600.0);

        // ----- SOl -----
        let sol_feed = &mut ctx.accounts.sol_feed;
        let sol_result = parse_switchboard_feed(&sol_feed)?;
        let sol_age = current_timestamp - sol_result.timestamp;

        msg!("SOL/USD ${:.2}", sol_result.price);
        msg!("Last Updated: {} seconds ago ({:.2}) hours", sol_age, sol_age as f64 / 3600.0);

        // ----- ETh -----
        let eth_feed = &mut ctx.accounts.eth_feed;
        let eth_result = parse_switchboard_feed(&eth_feed)?;
        let eth_age = current_timestamp - eth_result.timestamp;

        msg!("ETH/USD ${:.2}", eth_result.price);
        msg!("Last Updated: {} seconds ago ({:.2}) hours", eth_age, eth_age as f64 / 3600.0);

        Ok(())
    }
    pub fn get_prices_with_validation(ctx: Context<GetPrices>) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        let max_staleness: i64 = 300; // 5 minutes

        msg!("Current Timestamp: {}", current_timestamp);

        // ----- BTC -----
        let btc_feed = parse_switchboard_feed(&mut ctx.accounts.btc_feed)?;
        let btc_age = current_timestamp - btc_feed.timestamp;

        require!(btc_age <= max_staleness, ErrorCode::StaleFeed);

        msg!("BTC/USD: ${:.2}", btc_feed.price);
        msg!("Age: {}s", btc_age);

        // ----- SOL -----
        let sol_feed = parse_switchboard_feed(&mut ctx.accounts.sol_feed)?;
        let sol_age = current_timestamp - sol_feed.timestamp;

        require!(sol_age <= max_staleness, ErrorCode::StaleFeed );

        msg!("SOL/USD: ${:.2}", sol_feed.price);
        msg!("Age: {}s", sol_age);

        // ----- ETH -----
        let eth_feed = parse_switchboard_feed(&mut ctx.accounts.eth_feed)?;
        let eth_age = current_timestamp - eth_feed.timestamp;

        require!(eth_age <= max_staleness, ErrorCode::StaleFeed);

        msg!("ETH/USD: ${:.2}", eth_feed.price);
        msg!("Age: {}s", eth_age);

        Ok(())
    }

}

#[derive(Accounts)]
pub struct GetPrices<'info> {
    /// CHECK: Switchboard V2 BTC/USD
    pub btc_feed: AccountInfo<'info>,

    /// CHECK: Switchboard V2 SOL/USD
    pub sol_feed: AccountInfo<'info>,

    /// CHECK: Switchboard V2 ETH/USD
    pub eth_feed: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Price Feed Is Stale")]
    StaleFeed,
    #[msg("Invalid Price Feed Account")]
    InvalidPriceFeed,
    #[msg("Failed to parse price feed")]
    ParseError,
}

struct FeedResult{
    price: f64,
    timestamp: i64
}

fn parse_switchboard_feed(account_info: &AccountInfo) -> Result<FeedResult> {
    let data = account_info.try_borrow_data().map_err(|_| ErrorCode::ParseError)?;
    
    if data.len() < 237 {
        return Err(ErrorCode::InvalidPriceFeed.into());
    }

    let result_bytes: [u8; 16] = data[217..233]
        .try_into()
        .map_err(|_| ErrorCode::ParseError)?;
    let mantissa = i128::from_le_bytes(result_bytes);

    let scale_bytes: [u8; 4] = data[233..237]
        .try_into()
        .map_err(|_| ErrorCode::ParseError)?;
    let scale = u32::from_le_bytes(scale_bytes);

    let timestamp_bytes: [u8; 8] = data[229..237]
        .try_into()
        .map_err(|_| ErrorCode::ParseError)?;
    let timestamp = i64::from_le_bytes(timestamp_bytes);

    // Calculate price
    let price = mantissa as f64 / 10_f64.powi(scale as i32);

    Ok(FeedResult { price, timestamp })
}