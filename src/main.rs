use black_scholes::{BlackScholes, OptionType};

fn main() {
    println!("=== Black-Scholes Option Pricing Model ===\n");

    // Example parameters
    let spot_price = 100.0;
    let strike_price = 100.0;
    let time_to_expiry = 1.0; // 1 year
    let risk_free_rate = 0.05; // 5%
    let volatility = 0.2; // 20%
    let dividend_yield = 0.0; // No dividends

    // Create Black-Scholes model
    let bs = BlackScholes::new(
        spot_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        volatility,
        dividend_yield,
    )
    .expect("Failed to create Black-Scholes model");

    println!("Parameters:");
    println!("  Spot Price (S):        ${:.2}", spot_price);
    println!("  Strike Price (K):      ${:.2}", strike_price);
    println!("  Time to Expiry (T):    {:.2} years", time_to_expiry);
    println!("  Risk-Free Rate (r):    {:.2}%", risk_free_rate * 100.0);
    println!("  Volatility (σ):        {:.2}%", volatility * 100.0);
    println!("  Dividend Yield (q):    {:.2}%\n", dividend_yield * 100.0);

    // Calculate Call Option
    println!("--- Call Option ---");
    let call_price = bs.price(OptionType::Call);
    println!("Price: ${:.4}", call_price);
    
    let call_greeks = bs.greeks(OptionType::Call);
    println!("Greeks:");
    println!("  Delta:  {:.4}", call_greeks.delta);
    println!("  Gamma:  {:.4}", call_greeks.gamma);
    println!("  Vega:   {:.4}", call_greeks.vega);
    println!("  Theta:  {:.4}", call_greeks.theta);
    println!("  Rho:    {:.4}\n", call_greeks.rho);

    // Calculate Put Option
    println!("--- Put Option ---");
    let put_price = bs.price(OptionType::Put);
    println!("Price: ${:.4}", put_price);
    
    let put_greeks = bs.greeks(OptionType::Put);
    println!("Greeks:");
    println!("  Delta:  {:.4}", put_greeks.delta);
    println!("  Gamma:  {:.4}", put_greeks.gamma);
    println!("  Vega:   {:.4}", put_greeks.vega);
    println!("  Theta:  {:.4}", put_greeks.theta);
    println!("  Rho:    {:.4}\n", put_greeks.rho);

    // Verify Put-Call Parity
    let parity_left = call_price - put_price;
    let parity_right = spot_price * (-dividend_yield * time_to_expiry).exp()
        - strike_price * (-risk_free_rate * time_to_expiry).exp();
    println!("--- Put-Call Parity Check ---");
    println!("C - P = {:.4}", parity_left);
    println!("S*e^(-qT) - K*e^(-rT) = {:.4}", parity_right);
    println!("Difference: {:.6}\n", (parity_left - parity_right).abs());

    // Implied Volatility Example
    println!("--- Implied Volatility ---");
    match bs.implied_volatility(OptionType::Call, call_price, 100, 1e-6) {
        Ok(iv) => println!("Implied Vol from Call Price: {:.4} ({:.2}%)", iv, iv * 100.0),
        Err(e) => println!("Error calculating implied volatility: {}", e),
    }

    // Price sensitivity analysis
    println!("\n--- Price Sensitivity Analysis ---");
    println!("Spot Price | Call Price | Put Price");
    println!("-----------|-----------|----------");
    for s in (90..=110).step_by(5) {
        let bs_temp = BlackScholes::new(
            s as f64,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        )
        .unwrap();
        let c = bs_temp.price(OptionType::Call);
        let p = bs_temp.price(OptionType::Put);
        println!(" ${:>6}    | ${:>7.2}  | ${:>7.2}", s, c, p);
    }
}
