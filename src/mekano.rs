

pub struct Mekano<R, B, J> {
    pub root: BxBody<B, J>,
    data: R,
}

pub type BxJoint<B, J> = Box<Joint<B, J>>;
pub type BxBody<B, J> = Box<Body<B, J>>;

pub enum Body<B, J> {
    End(B),
    Segment(B, BxJoint<B, J>),
    Split(B, BxJoint<B, J>, BxJoint<B, J>),
}

impl<B, J> Body<B, J> {
    pub fn data<'a>(&'a self) -> &'a B {
        match self {
            &Body::End(ref d) => d,
            &Body::Segment(ref d, _) => d,
            &Body::Split(ref d, _, _) => d,
        }
    }
    pub fn data_mut<'a>(&'a mut self) -> &'a mut B {
        match self {
            &mut Body::End(ref mut d) => d,
            &mut Body::Segment(ref mut d, _) => d,
            &mut Body::Split(ref mut d, _, _) => d,
        }
    }
}

pub struct Joint<B, J> (J, BxBody<B, J>);

impl<R, B, J> Mekano<R, B, J> {
    pub fn new(data: R, root: BxBody<B, J>) -> Self {
        Mekano {
            data,
            root,
        }
    }
}

