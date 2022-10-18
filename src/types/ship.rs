#[derive(Clone, Copy, Debug)]
pub enum ShipSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Copy, Debug)]
pub struct Ship {
    pub damage: u32,
    pub size: ShipSize,
}

impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.size {
            ShipSize::Small => write!(f, "Small Ship"),
            ShipSize::Medium => write!(f, "Medium Ship"),
            ShipSize::Large => write!(f, "Large Ship"),
        }
    }
}

impl Ship {
    pub fn new(size: ShipSize) -> Self {
        Self { size, damage: 0 }
    }

    pub fn health(&self) -> f32 {
        let max = self.max_health() as f32;

        let health = (max - self.damage as f32) / max;
        health.max(0.)
    }

    pub fn repair_price(&self) -> u32 {
        let full_repair_price = self.price() as f32 * 0.8;

        (full_repair_price * (1. - self.health())).ceil() as u32
    }

    pub fn max_health(&self) -> u32 {
        match self.size {
            ShipSize::Small => 20,
            ShipSize::Medium => 50,
            ShipSize::Large => 100,
        }
    }

    pub fn resource_space(&self) -> u32 {
        match self.size {
            ShipSize::Small => 20,
            ShipSize::Medium => 50,
            ShipSize::Large => 100,
        }
    }

    pub fn price(&self) -> u32 {
        match self.size {
            ShipSize::Small => 2000,
            ShipSize::Medium => 4000,
            ShipSize::Large => 7500,
        }
    }

    pub fn construction_time(&self) -> u32 {
        match self.size {
            ShipSize::Small => 3,
            ShipSize::Medium => 4,
            ShipSize::Large => 5,
        }
    }

    pub fn repair_time(&self) -> u32 {
        let full_repair_time = self.construction_time() as f32;

        (full_repair_time * (1. - self.health())).ceil() as u32
    }
}
