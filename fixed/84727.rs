pub struct Color<T>(pub T);

pub struct Cell<Fg, Bg = Fg> {
    foreground: Color<Fg>,
    background: Color<Bg>,
}

pub trait Over<Bottom, Output> {
    fn over(self, bottom: Bottom) -> Output;
}

// Cell: Over<Color, Cell>
impl<C, Fg, Bg, NewFg, NewBg> Over<Color<C>, Cell<NewFg, NewBg>> for Cell<Fg, Bg>
where
    Fg: Over<C, NewFg>,
    Bg: Over<C, NewBg>,
{
    fn over(self, _: Color<C>) -> Cell<NewFg, NewBg> {
        todo!();
    }
}

// Cell: Over<Cell, Cell>
impl<TopFg, TopBg, BottomFg, BottomBg, NewFg, NewBg>
    Over<Cell<BottomFg, BottomBg>, Cell<NewFg, NewBg>> for Cell<TopFg, TopBg>
where
    Self: Over<Color<BottomBg>, Cell<NewFg, NewBg>>,
{
    fn over(self, bottom: Cell<BottomFg, BottomBg>) -> Cell<NewFg, NewBg> {
        self.over(bottom.background);
        todo!();
    }
}

// Cell: Over<&mut Cell, ()>
impl<'b, TopFg, TopBg, BottomFg, BottomBg> Over<&'b mut Cell<BottomFg, BottomBg>, ()>
    for Cell<TopFg, TopBg>
where
    Cell<TopFg, TopBg>: Over<Cell<BottomFg, BottomBg>, Cell<BottomFg, BottomBg>>,
{
    fn over(self, _: &'b mut Cell<BottomFg, BottomBg>) {
        todo!();
    }
}

// &Cell: Over<&mut Cell, ()>
impl<'t, 'b, TopFg, TopBg, BottomFg, BottomBg> Over<&'b mut Cell<BottomFg, BottomBg>, ()>
    for &'t Cell<TopFg, TopBg>
where
    Cell<TopFg, TopBg>: Over<Cell<BottomFg, BottomBg>, Cell<BottomFg, BottomBg>>,
{
    fn over(self, _: &'b mut Cell<BottomFg, BottomBg>) {
        todo!();
    }
}
