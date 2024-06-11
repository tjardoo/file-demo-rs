pub struct StateMachine<S> {
    pub state: S,
    pub shared_value: u32,
}

#[derive(Debug, PartialEq)]
pub struct Waiting;

#[derive(Debug, PartialEq)]
pub struct Filling;

#[derive(Debug, PartialEq)]
pub struct Done;

#[derive(Debug, PartialEq)]
pub struct Failed;

impl StateMachine<Waiting> {
    #[allow(dead_code)]
    pub fn new(shared_value: u32) -> Self {
        StateMachine {
            state: Waiting,
            shared_value,
        }
    }

    #[allow(dead_code)]
    pub fn fill(self) -> StateMachine<Filling> {
        StateMachine::<Filling>::from(self)
    }
}

impl StateMachine<Filling> {
    #[allow(dead_code)]
    pub fn complete(self) -> StateMachine<Done> {
        StateMachine::<Done>::from(self)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine() {
        let in_waiting = StateMachine::new(5);
        assert_eq!(in_waiting.state, Waiting);

        let in_filling = in_waiting.fill();
        assert_eq!(in_filling.state, Filling);

        let in_done = in_filling.complete();
        assert_eq!(in_done.state, Done);
        assert_eq!(in_done.shared_value, 5);
    }

    #[test]
    fn test_state_machine_without_functions() {
        let in_waiting = StateMachine::new(5);
        assert_eq!(in_waiting.state, Waiting);

        let in_filling = StateMachine::<Filling>::from(in_waiting);
        assert_eq!(in_filling.state, Filling);

        let in_done = StateMachine::<Done>::from(in_filling);
        assert_eq!(in_done.state, Done);
        assert_eq!(in_done.shared_value, 5);
    }
}
