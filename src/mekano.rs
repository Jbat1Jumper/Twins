

#[derive(Debug)]
pub enum Mekano<Data> {
    End(Data),
    Segment(Data, Box<Mekano<Data>>),
    Split(Data, Box<Mekano<Data>>, Box<Mekano<Data>>),
}

impl<Data> Mekano<Data> {
    pub fn data<'a>(&'a self) -> &'a Data {
        match self {
            &Mekano::End(ref d) => d,
            &Mekano::Segment(ref d, _) => d,
            &Mekano::Split(ref d, _, _) => d,
        }
    }
    pub fn data_mut<'a>(&'a mut self) -> &'a mut Data {
        match self {
            &mut Mekano::End(ref mut d) => d,
            &mut Mekano::Segment(ref mut d, _) => d,
            &mut Mekano::Split(ref mut d, _, _) => d,
        }
    }
}

