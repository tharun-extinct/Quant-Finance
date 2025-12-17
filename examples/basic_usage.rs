use black_scholes::{BlackScholes, OptionType};

fn main() {
    // Create a Black-Scholes model for a European call option
    let model = BlackScholes::new(
        100.0, // spot price: $100
        105.0, // strike price: $105
        0.5,   // time to expiry: 6 months
        0.05,  // risk-free rate: 5%
        0.25,  // volatility: 25%
        0.0,   // no dividends
    )
    .expect("Invalid parameters");

    // Calculate option prices
    let call_price = model.price(OptionType::Call);
    let put_price = model.price(OptionType::Put);

    println!("Call Option Price: ${:.2}", call_price);
    println!("Put Option Price:  ${:.2}", put_price);

    // Calculate Greeks for risk management
    let greeks = model.greeks(OptionType::Call);
    println!("\nCall Option Greeks:");
    println!("  Delta: {:.4} (sensitivity to spot price)", greeks.delta);
    println!("  Gamma: {:.4} (rate of change of delta)", greeks.gamma);
    println!("  Vega:  {:.4} (sensitivity to volatility)", greeks.vega);
    println!("  Theta: {:.4} (time decay per day)", greeks.theta);
    println!("  Rho:   {:.4} (sensitivity to interest rate)", greeks.rho);
}
