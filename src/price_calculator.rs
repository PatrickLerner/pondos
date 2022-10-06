pub struct PriceCalculator {
    pub base_price: u32,
    pub demand: u32,
    pub supply: u32,
}

const MIN_SHORTAGE_MOD: f32 = 0.8;
const MAX_SHORTAGE_MOD: f32 = 2.0;

impl PriceCalculator {
    fn shortage_mod(&self) -> f32 {
        let res = (self.demand as f32 - self.supply as f32) / self.demand as f32;

        f32::clamp(1. + res, MIN_SHORTAGE_MOD, MAX_SHORTAGE_MOD)
    }

    /// price for which player can buy
    pub fn buy_price(&self) -> u32 {
        (self.base_price as f32 * self.shortage_mod()).ceil() as u32
    }

    /// price for which player can sell
    pub fn sell_price(&self) -> u32 {
        let price = Self {
            base_price: self.base_price,
            demand: self.demand,
            supply: self.supply + 1,
        };

        // we would buy for price we could sell it at if we had one more
        price.buy_price()
    }
}
