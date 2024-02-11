use core::marker::PhantomData;

#[derive(Default)]
pub struct Pond {}

impl Pond {
    pub fn get<T>(&self, _out: Out<T>) -> &T {
        todo!();
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn pod<T>(&mut self, _pod: T) -> (Out<T>, In<T>) {
        todo!();
    }

    pub fn run<V: View>(self, _view: V) -> ! {
        todo!();
    }
}

pub trait View {
    fn view(&self, pond: &Pond) -> impl Render;
}

pub trait Render {
    fn render(&self);
}

pub struct In<T> {
    _ty: PhantomData<T>,
}

pub struct Out<T> {
    _ty: PhantomData<T>,
}

impl<T> Clone for Out<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Out<T> {}

impl<T> Out<T> {
    #[inline(always)]
    pub fn get(self, pond: &Pond) -> &T {
        pond.get(self)
    }
}
