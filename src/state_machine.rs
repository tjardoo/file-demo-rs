pub struct StateMachine<S> {
    state: S,
    pub shared_value: u32,
}

pub struct Waiting;

pub struct Filling;

pub struct Done;

impl StateMachine<Waiting> {
    pub fn new(shared_value: u32) -> Self {
        StateMachine {
            state: Waiting,
            shared_value,
        }
    }
}

impl From<StateMachine<Waiting>> for StateMachine<Filling> {
    fn from(state: StateMachine<Waiting>) -> StateMachine<Filling> {
        StateMachine {
            state: Filling,
            shared_value: state.shared_value,
        }
    }
}

impl From<StateMachine<Filling>> for StateMachine<Done> {
    fn from(state: StateMachine<Filling>) -> StateMachine<Done> {
        StateMachine {
            state: Done,
            shared_value: state.shared_value,
        }
    }
}
