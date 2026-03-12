# AMM Pinocchio

A lightweight Automated Market Maker (AMM) implementation for Solana built using the Pinocchio framework. This program implements a constant product AMM with LP tokens, swaps, and liquidity management.

## Features

- Constant product AMM (x * y = k)
- LP token minting and burning
- Token swaps with configurable fees
- Add/remove liquidity
- Built with Pinocchio for optimal performance

## AMM Formulas

### Swap Formula

The swap uses the constant product formula with fees:

```
amount_in_with_fee = amount_in * (10000 - fee_rate) / 10000
amount_out = (reserve_out * amount_in_with_fee) / (reserve_in + amount_in_with_fee)
```

Where:
- `fee_rate` is in basis points (1 basis point = 0.01%, 10000 basis points = 100%)
- Fees are deducted from the input amount before calculating output

### Add Liquidity Formula

**Initial liquidity:**
```
lp_tokens = sqrt(amount_a * amount_b)
```

**Subsequent liquidity:**
```
lp_a = (amount_a * total_lp_supply) / reserve_a
lp_b = (amount_b * total_lp_supply) / reserve_b
lp_tokens = min(lp_a, lp_b)
```

### Withdraw Liquidity Formula

```
amount_a_out = (lp_amount * reserve_a) / total_lp_supply
amount_b_out = (lp_amount * reserve_b) / total_lp_supply
```

## Build and Maintain 

Avhi ([Web](https://avhi.in) | [X](https://x.com/avhidotsol))
