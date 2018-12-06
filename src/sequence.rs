
impl Sequence {
    fn new() -> Sequence {
        Sequence {
            current: 0,
        }
    }
    fn step<'a, 'se, S, C>(&mut self, state: &'a mut S, context: &'a mut C) -> SecuenceExecuter<'a, S, C> {
        SecuenceExecuter<'a, S, C> {
            execute_at_step: 0,
            before: self.steps,
            now: self.steps + 1,
        }
        self.steps += 1;
    }
}

struct SecuenceExecuter<'a, S, C> {
    before: u32,
    now: u32,
    execute_at_step: u32,
    state: &'a mut S,
    context: &'a mut C,
}

impl SecuenceExecuter<S, C> {
    pub fn then<F>(self, c: F) -> Self
    where
        F: FnOnce(&mut S, &mut C),
    {
        if self.before <= self.execute_at_step && self.execute_at_step < self.now {
            c(self.state, self.context);
        }
        SecuenceExecuter {
            ..self
        }
    }
    pub fn wait(self, steps: u32) -> Self {
        SecuenceExecuter {
            self.execute_at_step += steps,
            ..self
        }
    }
}

impl<C> Update<C> for Sequence {
    fn update(&mut self, &mut C) {
    }
}
