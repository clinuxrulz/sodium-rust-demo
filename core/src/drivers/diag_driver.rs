pub trait DiagDriver {
    fn log_string(&mut self, msg: String);
}

pub trait Log {
    fn log<S: ToString>(&mut self, msg: S);
}

impl Log for DiagDriver {
    fn log<S: ToString>(&mut self, msg: S) {
        self.log_string(msg.to_string());
    }
}

impl<DD: DiagDriver> Log for DD {
    fn log<S: ToString>(&mut self, msg: S) {
        self.log_string(msg.to_string());
    }
}
