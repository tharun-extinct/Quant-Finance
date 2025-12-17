use std::f64::consts::{PI, SQRT_2};

/// Type of option: Call or Put
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptionType {
    Call,
    Put,
}

/// Greeks for option sensitivity analysis
#[derive(Debug, Clone, Copy)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    pub rho: f64,
}

/// Black-Scholes Option Pricing Model
#[derive(Debug, Clone, Copy)]
pub struct BlackScholes {
    /// Current price of the underlying asset
    pub spot_price: f64,
    /// Strike price of the option
    pub strike_price: f64,
    /// Time to expiration in years
    pub time_to_expiry: f64,
    /// Risk-free interest rate (annual)
    pub risk_free_rate: f64,
    /// Volatility of the underlying asset (annual)
    pub volatility: f64,
    /// Dividend yield (annual, optional - defaults to 0)
    pub dividend_yield: f64,
}

impl BlackScholes {
    /// Create a new Black-Scholes model instance
    ///
    /// # Arguments
    /// * `spot_price` - Current price of the underlying asset (S)
    /// * `strike_price` - Strike price of the option (K)
    /// * `time_to_expiry` - Time to expiration in years (T)
    /// * `risk_free_rate` - Risk-free interest rate as decimal (r)
    /// * `volatility` - Volatility of underlying as decimal (σ)
    /// * `dividend_yield` - Dividend yield as decimal (q), optional
    pub fn new(
        spot_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        dividend_yield: f64,
    ) -> Result<Self, String> {
        if spot_price <= 0.0 {
            return Err("Spot price must be positive".to_string());
        }
        if strike_price <= 0.0 {
            return Err("Strike price must be positive".to_string());
        }
        if time_to_expiry <= 0.0 {
            return Err("Time to expiry must be positive".to_string());
        }
        if volatility <= 0.0 {
            return Err("Volatility must be positive".to_string());
        }

        Ok(BlackScholes {
            spot_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        })
    }

    /// Calculate d1 parameter in Black-Scholes formula
    fn d1(&self) -> f64 {
        let numerator = (self.spot_price / self.strike_price).ln()
            + (self.risk_free_rate - self.dividend_yield + 0.5 * self.volatility.powi(2))
                * self.time_to_expiry;
        let denominator = self.volatility * self.time_to_expiry.sqrt();
        numerator / denominator
    }

    /// Calculate d2 parameter in Black-Scholes formula
    fn d2(&self) -> f64 {
        self.d1() - self.volatility * self.time_to_expiry.sqrt()
    }

    /// Standard normal cumulative distribution function (CDF)
    /// Approximation using the error function
    fn norm_cdf(x: f64) -> f64 {
        0.5 * (1.0 + Self::erf(x / SQRT_2))
    }

    /// Standard normal probability density function (PDF)
    fn norm_pdf(x: f64) -> f64 {
        (-0.5 * x.powi(2)).exp() / (2.0 * PI).sqrt()
    }

    /// Error function approximation using Abramowitz and Stegun formula
    /// Accurate to 1.5 × 10^-7
    fn erf(x: f64) -> f64 {
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let p = 0.3275911;

        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        let x = x.abs();

        let t = 1.0 / (1.0 + p * x);
        let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

        sign * y
    }

    /// Calculate option price
    ///
    /// # Arguments
    /// * `option_type` - Type of option (Call or Put)
    ///
    /// # Returns
    /// Option price
    pub fn price(&self, option_type: OptionType) -> f64 {
        let d1 = self.d1();
        let d2 = self.d2();

        match option_type {
            OptionType::Call => {
                self.spot_price * (-self.dividend_yield * self.time_to_expiry).exp() * Self::norm_cdf(d1)
                    - self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp() * Self::norm_cdf(d2)
            }
            OptionType::Put => {
                self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp() * Self::norm_cdf(-d2)
                    - self.spot_price * (-self.dividend_yield * self.time_to_expiry).exp() * Self::norm_cdf(-d1)
            }
        }
    }

    /// Calculate all Greeks for the option
    ///
    /// # Arguments
    /// * `option_type` - Type of option (Call or Put)
    ///
    /// # Returns
    /// Greeks struct containing delta, gamma, vega, theta, and rho
    pub fn greeks(&self, option_type: OptionType) -> Greeks {
        let d1 = self.d1();
        let d2 = self.d2();
        let sqrt_t = self.time_to_expiry.sqrt();
        let discount = (-self.risk_free_rate * self.time_to_expiry).exp();
        let dividend_discount = (-self.dividend_yield * self.time_to_expiry).exp();

        // Delta: Rate of change of option price with respect to underlying price
        let delta = match option_type {
            OptionType::Call => dividend_discount * Self::norm_cdf(d1),
            OptionType::Put => -dividend_discount * Self::norm_cdf(-d1),
        };

        // Gamma: Rate of change of delta with respect to underlying price
        let gamma = (dividend_discount * Self::norm_pdf(d1)) 
            / (self.spot_price * self.volatility * sqrt_t);

        // Vega: Rate of change of option price with respect to volatility (divided by 100)
        let vega = (self.spot_price * dividend_discount * Self::norm_pdf(d1) * sqrt_t) / 100.0;

        // Theta: Rate of change of option price with respect to time (per day)
        let theta = match option_type {
            OptionType::Call => {
                let term1 = -(self.spot_price * Self::norm_pdf(d1) * self.volatility * dividend_discount)
                    / (2.0 * sqrt_t);
                let term2 = self.dividend_yield * self.spot_price * Self::norm_cdf(d1) * dividend_discount;
                let term3 = self.risk_free_rate * self.strike_price * discount * Self::norm_cdf(d2);
                (term1 - term2 + term3) / 365.0
            }
            OptionType::Put => {
                let term1 = -(self.spot_price * Self::norm_pdf(d1) * self.volatility * dividend_discount)
                    / (2.0 * sqrt_t);
                let term2 = self.dividend_yield * self.spot_price * Self::norm_cdf(-d1) * dividend_discount;
                let term3 = self.risk_free_rate * self.strike_price * discount * Self::norm_cdf(-d2);
                (term1 + term2 - term3) / 365.0
            }
        };

        // Rho: Rate of change of option price with respect to interest rate (divided by 100)
        let rho = match option_type {
            OptionType::Call => {
                (self.strike_price * self.time_to_expiry * discount * Self::norm_cdf(d2)) / 100.0
            }
            OptionType::Put => {
                -(self.strike_price * self.time_to_expiry * discount * Self::norm_cdf(-d2)) / 100.0
            }
        };

        Greeks {
            delta,
            gamma,
            vega,
            theta,
            rho,
        }
    }

    /// Calculate implied volatility using Newton-Raphson method
    ///
    /// # Arguments
    /// * `option_type` - Type of option (Call or Put)
    /// * `market_price` - Observed market price of the option
    /// * `max_iterations` - Maximum number of iterations (default: 100)
    /// * `tolerance` - Convergence tolerance (default: 1e-6)
    ///
    /// # Returns
    /// Implied volatility or error if not converged
    pub fn implied_volatility(
        &self,
        option_type: OptionType,
        market_price: f64,
        max_iterations: usize,
        tolerance: f64,
    ) -> Result<f64, String> {
        let mut vol = 0.3; // Initial guess
        
        for _ in 0..max_iterations {
            let mut bs = *self;
            bs.volatility = vol;
            
            let price = bs.price(option_type);
            let vega = bs.greeks(option_type).vega * 100.0; // Adjust for scaling
            
            if vega.abs() < 1e-10 {
                return Err("Vega too small, cannot converge".to_string());
            }
            
            let diff = market_price - price;
            
            if diff.abs() < tolerance {
                return Ok(vol);
            }
            
            vol += diff / vega;
            
            // Ensure volatility stays positive
            if vol <= 0.0 {
                vol = 0.001;
            }
        }
        
        Err("Failed to converge".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_scholes_call() {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let call_price = bs.price(OptionType::Call);
        
        // Expected value approximately 10.45
        assert!((call_price - 10.45).abs() < 0.1);
    }

    #[test]
    fn test_black_scholes_put() {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let put_price = bs.price(OptionType::Put);
        
        // Expected value approximately 5.57
        assert!((put_price - 5.57).abs() < 0.1);
    }

    #[test]
    fn test_put_call_parity() {
        // Put-Call Parity: C - P = S * e^(-qT) - K * e^(-rT)
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let call = bs.price(OptionType::Call);
        let put = bs.price(OptionType::Put);
        
        let left = call - put;
        let right = bs.spot_price * (-bs.dividend_yield * bs.time_to_expiry).exp()
            - bs.strike_price * (-bs.risk_free_rate * bs.time_to_expiry).exp();
        
        assert!((left - right).abs() < 0.01);
    }

    #[test]
    fn test_greeks() {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let greeks = bs.greeks(OptionType::Call);
        
        // Delta should be between 0 and 1 for calls
        assert!(greeks.delta > 0.0 && greeks.delta < 1.0);
        
        // Gamma should be positive
        assert!(greeks.gamma > 0.0);
        
        // Vega should be positive
        assert!(greeks.vega > 0.0);
    }

    #[test]
    fn test_invalid_parameters() {
        assert!(BlackScholes::new(-100.0, 100.0, 1.0, 0.05, 0.2, 0.0).is_err());
        assert!(BlackScholes::new(100.0, -100.0, 1.0, 0.05, 0.2, 0.0).is_err());
        assert!(BlackScholes::new(100.0, 100.0, -1.0, 0.05, 0.2, 0.0).is_err());
        assert!(BlackScholes::new(100.0, 100.0, 1.0, 0.05, -0.2, 0.0).is_err());
    }

    #[test]
    fn test_implied_volatility() {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let call_price = bs.price(OptionType::Call);
        
        let implied_vol = bs.implied_volatility(OptionType::Call, call_price, 100, 1e-6).unwrap();
        
        // Should recover the original volatility
        assert!((implied_vol - 0.2).abs() < 0.001);
    }
}
