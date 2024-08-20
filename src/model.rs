#[derive(Debug, Default)]
pub enum SuperState {
    #[default]
    A,
    B,
    C,
    D,
    E,
}

#[derive(Debug, Default)]
pub enum Transition {
    #[default]
    AtoA,
    AtoB,
    BtoB,
    BtoC,
    BtoD,
    BtoE,
    CtoC,
    CtoE,
    DtoD,
    DtoE,
    EtoE,
    EtoA,
}

#[derive(Debug)]
pub struct StateMachine {
    pub super_state: SuperState,
    pub transition: Transition,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            super_state: Default::default(),
            transition: Default::default(),
        }
    }
}

impl StateMachine {
    pub fn update_state_machine(&mut self) {
        match self.transition {
            Transition::AtoA => {
                self.super_state = SuperState::A;
                self.process_a_to_a();
            }
            Transition::AtoB => {
                self.super_state = SuperState::B;
                self.process_a_to_b();
            }
            Transition::BtoB => {
                self.super_state = SuperState::B;
                self.process_b_to_b();
            }
            Transition::BtoC => {
                self.super_state = SuperState::C;
                self.process_b_to_c();
            }
            Transition::BtoD => {
                self.super_state = SuperState::D;
                self.process_b_to_d();
            }
            Transition::BtoE => {
                self.super_state = SuperState::E;
                self.process_b_to_e();
            }
            Transition::CtoC => {
                self.super_state = SuperState::C;
                self.process_c_to_c();
            }
            Transition::CtoE => {
                self.super_state = SuperState::E;
                self.process_c_to_e();
            }
            Transition::DtoD => {
                self.super_state = SuperState::D;
                self.process_d_to_d();
            }
            Transition::DtoE => {
                self.super_state = SuperState::E;
                self.process_d_to_e();
            }
            Transition::EtoE => {
                self.super_state = SuperState::E;
                self.process_e_to_e();
            }
            Transition::EtoA => {
                self.super_state = SuperState::A;
                self.process_e_to_a();
            }
        }
    }

    fn process_a_to_a(&self) {}
    fn process_a_to_b(&self) {}
    fn process_b_to_b(&self) {}
    fn process_b_to_c(&self) {}
    fn process_b_to_d(&self) {}
    fn process_b_to_e(&self) {}
    fn process_c_to_c(&self) {}
    fn process_c_to_e(&self) {}
    fn process_d_to_d(&self) {}
    fn process_d_to_e(&self) {}
    fn process_e_to_e(&self) {}
    fn process_e_to_a(&self) {}
}
