enum Event {
    ENTRY,
    EXIT,
}

struct Data;

struct State<'a> {
    f: Fn(&mut Data, &Event) -> &'a Signal<'a>,
}

enum Signal<'a> {
    UNHANDLED,
    HANDLED,
    TRAN(Fn(&mut Data, &Event) -> &'a Signal<'a>),
}

struct FSM<'a>  {
    current : Option<&'a State<'a>>,
    data : Data,
}

impl <'a> FSM<'a> {

    pub fn start(&mut self) {
        match self.current {
            None => panic!("Ola !"),
            Some(current) => {(current.f)(&mut self.data, &Event::ENTRY);},
            }

    }

    pub fn event(&mut self,event : Event) {
        let signal = (self.current.unwrap().f)(&mut self.data, &event);
    }
}

fn main() {
}
