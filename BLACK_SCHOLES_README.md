# Black-Scholes Option Pricing Module (Rust)

A comprehensive, from-scratch implementation of the Black-Scholes option pricing model in Rust, including:
- European call and put option pricing
- All Greeks (Delta, Gamma, Vega, Theta, Rho)
- Implied volatility calculation using Newton-Raphson method
- Put-Call parity verification
- Comprehensive unit tests

## Features

### Core Functionality
- **Option Pricing**: Calculate theoretical prices for European call and put options
- **Greeks Calculation**: Full suite of risk sensitivities
- **Implied Volatility**: Reverse-engineer volatility from market prices
- **Pure Rust Implementation**: No external dependencies, all math implemented from scratch

### Mathematical Implementation
- Standard normal CDF using error function approximation (Abramowitz & Stegun)
- Standard normal PDF for probability calculations
- Dividend yield support for stocks with dividends
- Numerical stability for edge cases

## Installation

This is a standalone Rust project. To use it:

```bash
# Clone or navigate to the project directory
cd Quant-Finance

# Build the project
cargo build --release

# Run the main example
cargo run --release

# Run tests
cargo test

# Run basic usage example
cargo run --example basic_usage
```

## Usage

### Basic Example

```rust
use black_scholes::{BlackScholes, OptionType};

fn main() {
    // Create a Black-Scholes model
    let model = BlackScholes::new(
        100.0, // spot price: $100
        105.0, // strike price: $105
        0.5,   // time to expiry: 6 months (0.5 years)
        0.05,  // risk-free rate: 5% per annum
        0.25,  // volatility: 25% per annum
        0.0,   // dividend yield: 0%
    ).expect("Invalid parameters");

    // Calculate option prices
    let call_price = model.price(OptionType::Call);
    let put_price = model.price(OptionType::Put);

    println!("Call Price: ${:.2}", call_price);
    println!("Put Price:  ${:.2}", put_price);
}
```

### Greeks Calculation

```rust
// Calculate all Greeks at once
let greeks = model.greeks(OptionType::Call);

println!("Delta: {:.4}", greeks.delta);  // Sensitivity to spot price
println!("Gamma: {:.4}", greeks.gamma);  // Rate of change of delta
println!("Vega:  {:.4}", greeks.vega);   // Sensitivity to volatility
println!("Theta: {:.4}", greeks.theta);  // Time decay (per day)
println!("Rho:   {:.4}", greeks.rho);    // Sensitivity to interest rate
```

### Implied Volatility

```rust
// Given a market price, find the implied volatility
let market_price = 8.50;
let implied_vol = model.implied_volatility(
    OptionType::Call,
    market_price,
    100,   // max iterations
    1e-6   // tolerance
).expect("Failed to converge");

println!("Implied Volatility: {:.2}%", implied_vol * 100.0);
```

## Black-Scholes Formula

### Call Option Price
```
C = S * e^(-qT) * N(d1) - K * e^(-rT) * N(d2)
```

### Put Option Price
```
P = K * e^(-rT) * N(-d2) - S * e^(-qT) * N(-d1)
```

### Where:
```
d1 = [ln(S/K) + (r - q + σ²/2)T] / (σ√T)
d2 = d1 - σ√T
```

- **S** = Current spot price
- **K** = Strike price
- **T** = Time to expiration (years)
- **r** = Risk-free interest rate
- **σ** = Volatility (sigma)
- **q** = Dividend yield
- **N()** = Cumulative standard normal distribution

## Greeks Formulas

### Delta (Δ)
Rate of change of option price with respect to underlying price:
- Call: `Δ = e^(-qT) * N(d1)`
- Put: `Δ = -e^(-qT) * N(-d1)`

### Gamma (Γ)
Rate of change of delta with respect to underlying price:
```
Γ = [e^(-qT) * n(d1)] / (S * σ * √T)
```

### Vega (ν)
Sensitivity to volatility (per 1% change):
```
ν = S * e^(-qT) * n(d1) * √T / 100
```

### Theta (Θ)
Time decay (per day):
- Call: `Θ = [-S*n(d1)*σ*e^(-qT)/(2√T) - qSN(d1)e^(-qT) + rKe^(-rT)N(d2)] / 365`
- Put: `Θ = [-S*n(d1)*σ*e^(-qT)/(2√T) + qSN(-d1)e^(-qT) - rKe^(-rT)N(-d2)] / 365`

### Rho (ρ)
Sensitivity to interest rate (per 1% change):
- Call: `ρ = K * T * e^(-rT) * N(d2) / 100`
- Put: `ρ = -K * T * e^(-rT) * N(-d2) / 100`

## API Reference

### `BlackScholes` Struct

```rust
pub struct BlackScholes {
    pub spot_price: f64,        // Current price of underlying
    pub strike_price: f64,       // Strike price
    pub time_to_expiry: f64,     // Time to expiration (years)
    pub risk_free_rate: f64,     // Risk-free rate (annual)
    pub volatility: f64,         // Volatility (annual)
    pub dividend_yield: f64,     // Dividend yield (annual)
}
```

### Methods

#### `new()`
Create a new Black-Scholes model with validation.

**Returns:** `Result<BlackScholes, String>`

#### `price(option_type: OptionType) -> f64`
Calculate the theoretical option price.

#### `greeks(option_type: OptionType) -> Greeks`
Calculate all Greeks for risk management.

#### `implied_volatility(option_type, market_price, max_iterations, tolerance) -> Result<f64, String>`
Calculate implied volatility from market price using Newton-Raphson method.

### `OptionType` Enum

```rust
pub enum OptionType {
    Call,
    Put,
}
```

### `Greeks` Struct

```rust
pub struct Greeks {
    pub delta: f64,  // Δ: Sensitivity to spot price
    pub gamma: f64,  // Γ: Rate of change of delta
    pub vega: f64,   // ν: Sensitivity to volatility
    pub theta: f64,  // Θ: Time decay
    pub rho: f64,    // ρ: Sensitivity to interest rate
}
```

## Testing

The module includes comprehensive unit tests:

```bash
cargo test
```

Tests include:
- Call and put option pricing
- Put-call parity verification
- Greeks calculation
- Parameter validation
- Implied volatility convergence

## Example Output

```
=== Black-Scholes Option Pricing Model ===

Parameters:
  Spot Price (S):        $100.00
  Strike Price (K):      $100.00
  Time to Expiry (T):    1.00 years
  Risk-Free Rate (r):    5.00%
  Volatility (σ):        20.00%
  Dividend Yield (q):    0.00%

--- Call Option ---
Price: $10.4506
Greeks:
  Delta:  0.6368
  Gamma:  0.0188
  Vega:   0.3746
  Theta:  -0.0202
  Rho:    0.5324

--- Put Option ---
Price: $5.5735
Greeks:
  Delta:  -0.3632
  Gamma:  0.0188
  Vega:   0.3746
  Theta:  -0.0153
  Rho:    -0.4188
```

## Assumptions & Limitations

This implementation assumes:
- **European options** (can only be exercised at expiration)
- **No transaction costs** or taxes
- **Constant volatility** over the option's life
- **Log-normal distribution** of stock prices
- **Continuous trading** is possible
- **No arbitrage opportunities**

For American options or more complex derivatives, consider numerical methods like binomial trees or Monte Carlo simulation.

## Performance

- Pure Rust implementation with no external dependencies
- O(1) time complexity for pricing and Greeks
- Efficient Newton-Raphson for implied volatility
- Suitable for high-frequency calculations

## License

This is an educational implementation for quantitative finance learning.

## References

1. Black, F., & Scholes, M. (1973). "The Pricing of Options and Corporate Liabilities"
2. Hull, J. C. "Options, Futures, and Other Derivatives"
3. Wilmott, P. "Paul Wilmott on Quantitative Finance"

## Contributing

Feel free to extend this module with:
- American option pricing (binomial trees, finite differences)
- Exotic options (Asian, Barrier, Lookback)
- Monte Carlo simulation
- More Greeks (Vanna, Volga, Charm, etc.)
- Smile/surface modeling
