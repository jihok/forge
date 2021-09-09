# FORGE PROTOCOL

Forge aims to upgrade Pylon deposit pools by launching 2 companion pools with unique benefits for each Pylon pool: a yield-boosted pool and a yield-insured pool. Users must also lock some $FORGE to deposit into one of these pools. Depositors in the yield-insured pool redirect most of their Pylon rewards to the yield-boosted depositors. However, the locked $FORGE of yield-boosters is in return at risk--the yield-insured depositors may redeem their reward tokens against it at any time.

![alt text](docs/assets/low_risk_deposit.png)

<!-- TODO: concat this and high-risk diagram + show receipt of locked $FORGE and DP Tokens -->

The $FORGE locks should last for some predetermined time after the underlying Pylon pools end. As long as the lock persists, redemptions should be respected.
![alt text](docs/assets/redeem.png)

# $FORGE

Since each of these pools depends on the other, users should be incentivized to deposit into the lagging pool when they are unbalanced.
For projects where people are generally bullish, we expect more high-risk depositors, which should in turn increase the floor rate for low-risk depositors.

<!-- TODO: diagram -->

For projects where people are generally bearish, we expect more low-risk depositors, which should in turn increase the yield for high-risk depositors.

<!-- TODO: diagram -->

To get the ball rolling, Forge treasury may need to seed initial liquidity in each pool.

## Governance Parameters:

- Initial seeding
- Lock time
- Boost rate (rate of change + max)
- Insurance rate (rate of change + max)
- Fees

# Current state

![alt text](docs/assets/user_deposit.png)

The code in this repository defines a smart contract that users can deposit funds into. Each implementation of this contract should be 1:1 with a corresponding Pylon Pool contract where user funds will be directed to under the hood. As far as Pylon is concerned, Forge would just be one large depositor (subject to change if Pylon enacts deposit limits per user, but a whitelist would allow for this in this case).

# Run/Deploy

- `cargo wasm` to build
- `cargo run-script optimize` to minimize the WASM binary
- `cargo schema` to generate the JSON schema
- `npm run start` to deploy the contract to `tequila-0004` testnet

# Theoretical Launch Roadmap

## 1. Seed liquidity

A launch via Pylon makes sense since Forge's primary purpose is to further incentivize utilization of Pylon Gateway pools beyond its current appeal.

## 2. Open Incentivized LP Pools

Inflation should allow for $FORGE to land in as many wallets as possible. The buy-in for risk-takers should be low, whereas risk-averse users will be afforded an opportunity to accumulate.

## 3. Locked $FORGE tokens

While low-risk pools can open without locking $FORGE, they need high-risk deposits to have any value, and $FORGE needs to be locked for high-risk pools to open.

## 4. Deposit Pools

Technically redemption doesn't need to be implemented at this stage. However it may make sense to have it ready/public at the same time as deposit pools because otherwise the low-risk depositors would actually be at risk.
