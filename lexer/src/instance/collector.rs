use super::Instance;

enum CollectorState {
    String,
    Numeric
}

pub struct Collector {
    state: CollectorState
}

impl Collector {
    pub fn tokenize<S: Into<String>>(&mut self, instance: &mut Instance, input: S) {
        let input: String = input.into();
    }
}