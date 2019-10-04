fn main() {
    enum Msg {
        Start(usize),
        Measure([u8]),
    }


    let _ = Msg::Start;
}
