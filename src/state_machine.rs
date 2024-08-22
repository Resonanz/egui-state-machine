// Three things are needed for a state machine:
//
// 1. The STATE the machine is currently in
//    -- each state is unique, thus use enum for each state
// 2. The desired transition from STATE_NOW to STATE_NEXT
//    -- each transition is unique, so use enum for each transition
// 3. The validity of the transition
//    -- need to check if transition is valid
//    -- option 1. could use an const array of FROM/TO tuples to test for valid states
//    -- option 2. could check current state against transition FROM/TO to ensure valid transition
//
// We should match on the transition rather than the state as the transition is
// what is interesting because it results from a desired change in the system.

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
pub struct StateMachine {
    // This PUB struct (and its impl) is available outside this module.
    prev_state: State, // This struct field is private.
    curr_state: State,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            prev_state: Default::default(),
            curr_state: Default::default(),
        }
    }
}

impl StateMachine {
    pub fn get_curr_state(&self) -> &State {
        &self.curr_state
    }

    pub fn update_state_machine(&mut self, transition: &Transition) {
        // Match on transition, not state
        match transition {
            // Most times there will be no transition
            Transition::NoTransition => {}
            // First we match on the transition, then
            // check the current state where we are
            // transitioning away from. This allows a
            // transition e.g. AtoB to take place
            // only if we are beginning in State::A.
            Transition::AtoA => match self.curr_state {
                State::A => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::A;
                    self.process_a_to_a(&transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::AtoB => match self.curr_state {
                State::A => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::B;
                    self.process_a_to_b(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::BtoB => match self.curr_state {
                State::B => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::B;
                    self.process_b_to_b(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::BtoC => match self.curr_state {
                State::B => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::C;
                    self.process_b_to_c(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::BtoD => match self.curr_state {
                State::B => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::D;
                    self.process_b_to_d(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::BtoE => match self.curr_state {
                State::B => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::E;
                    self.process_b_to_e(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::CtoC => match self.curr_state {
                State::C => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::C;
                    self.process_c_to_c(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::CtoE => match self.curr_state {
                State::C => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::E;
                    self.process_c_to_e(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::DtoD => match self.curr_state {
                State::D => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::D;
                    self.process_d_to_d(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::DtoE => match self.curr_state {
                State::D => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::E;
                    self.process_d_to_e(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::EtoE => match self.curr_state {
                State::E => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::E;
                    self.process_e_to_e(transition);
                }
                _ => self.do_panic(transition),
            },
            Transition::EtoA => match self.curr_state {
                State::E => {
                    self.prev_state = self.curr_state.clone();
                    self.curr_state = State::A;
                    self.process_e_to_a(transition);
                }
                _ => self.do_panic(transition),
            },
        }
    }

    // P A N I C !!!
    fn do_panic(&self, trans: &Transition) {
        panic!(
            "trans = {:?}, but our starting state = {:?}",
            trans, self.curr_state
        )
    }

    // Process stuff
    fn process_a_to_a(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_a_to_b(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_b_to_b(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_b_to_c(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_b_to_d(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_b_to_e(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_c_to_c(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_c_to_e(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_d_to_d(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_d_to_e(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_e_to_e(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
    fn process_e_to_a(&self, trans: &Transition) {
        println!(
            "trans = {:?}, self.prev_state = {:?}, self.curr_state = {:?}",
            trans, self.prev_state, self.curr_state
        );
    }
}
