#[derive(Clone, Debug, Default)]
pub enum State {
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
    NoTransition,
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
pub struct StateMachine { // This PUB struct (and its impl) is available outside this module.
    state: State, // This struct field is private.
    prev_state: State,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            state: Default::default(),
            prev_state: Default::default(),
        }
    }
}

impl StateMachine {
    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn update_state_machine(&mut self, trans: &Transition) {

        // Save previous state
        self.prev_state = self.state.clone();  // What to use prev_state for?

        // Match on new state
        match trans {
            Transition::NoTransition => {
                self.process_no_transition();
            }
            Transition::AtoA => {
                self.state = State::A;
                self.process_a_to_a();
            }
            Transition::AtoB => {
                self.state = State::B;
                self.process_a_to_b();
            }
            Transition::BtoB => {
                self.state = State::B;
                self.process_b_to_b();
            }
            Transition::BtoC => {
                self.state = State::C;
                self.process_b_to_c();
            }
            Transition::BtoD => {
                self.state = State::D;
                self.process_b_to_d();
            }
            Transition::BtoE => {
                self.state = State::E;
                self.process_b_to_e();
            }
            Transition::CtoC => {
                self.state = State::C;
                self.process_c_to_c();
            }
            Transition::CtoE => {
                self.state = State::E;
                self.process_c_to_e();
            }
            Transition::DtoD => {
                self.state = State::D;
                self.process_d_to_d();
            }
            Transition::DtoE => {
                self.state = State::E;
                self.process_d_to_e();
            }
            Transition::EtoE => {
                self.state = State::E;
                self.process_e_to_e();
            }
            Transition::EtoA => {
                self.state = State::A;
                self.process_e_to_a();
            }
        }
    }

    fn process_no_transition(&self) {}
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
