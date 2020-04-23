pub trait Infectable {
    fn get_immunity(self) -> i32;
    fn become_infected(&mut self);
    fn needs_doctor(&mut self) -> bool;
    fn cure(&mut self);
}

#[derive(Copy, Clone)]
pub struct InfectionParams {
    immunity: i32,
    health: i32,
    // has_symptops : bool,
}

impl InfectionParams {
    pub fn get_immunity(self) -> i32 {
        self.immunity
    }

    pub fn is_healthy(self) -> bool {
        self.health == self.immunity
    }

    pub fn helth_to_imm(self) -> f32 {
        (self.health as f32) / (self.immunity as f32)
    }

    pub fn infect(&mut self) {
        if self.health > 0 {
            self.health -= 1;
        }
    }

    pub fn recover(&mut self) {
        self.health = self.immunity;
    }

    pub fn cure(&mut self) {
        if self.health < self.immunity {
            self.health += 1;
        }
    }

    pub fn needs_doctor(self) -> bool {
        self.health <= 0
    }
}

impl Default for InfectionParams {
    fn default() -> Self {
        InfectionParams {
            immunity: 10,
            health: 10,
        }
    }
}
