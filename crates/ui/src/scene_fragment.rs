use alloc::rc::Rc;
use core::cell::Cell;
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::{Bound, Cells, Point, Scene};

/// TODO: docs.
pub struct SceneFragment<'scene> {
    /// TODO: docs.
    ptr: NonNull<Scene>,

    /// The origin of the fragment.
    ///
    /// The area of the scene covered by this fragment is the rectangle given
    /// by [`Self::size`] with its top-left corner at this point.
    origin: Point<Cells>,

    /// The size of the fragment.
    size: Bound<Cells>,

    /// Whether the fragment is currently borrowed. This is needed to make
    /// accessing the scene via a raw pointer safe.
    borrow: Rc<Cell<Borrow>>,

    _lifetime: PhantomData<&'scene mut Scene>,
}

enum Borrow {
    Borrowed,
    NotBorrowed,
}

impl<'scene> SceneFragment<'scene> {
    /// TODO: docs
    #[inline]
    pub fn cutout<C: Cutout>(self, cutout: C) -> (Self, C::Cutout<'scene>) {
        cutout.cutout(self)
    }

    /// TODO: docs
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }

    /// TODO: docs
    #[inline]
    pub fn height(&self) -> Cells {
        self.size.height()
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn new(scene: &'scene mut Scene) -> Self {
        Self {
            size: scene.size(),
            ptr: NonNull::from(scene),
            origin: Point::origin(),
            borrow: Rc::new(Cell::new(Borrow::NotBorrowed)),
            _lifetime: PhantomData,
        }
    }

    /// TODO: docs
    #[inline]
    pub fn split_x(mut self, split_at: Cells) -> (Self, Self) {
        let mut bottom_origin = self.origin;
        *bottom_origin.y_mut() += split_at;

        let mut bottom_size = self.size;
        *bottom_size.height_mut() -= split_at;

        *self.size.height_mut() = split_at;

        let bottom_fragment = Self {
            ptr: self.ptr,
            origin: bottom_origin,
            size: bottom_size,
            borrow: self.borrow.clone(),
            _lifetime: PhantomData,
        };

        (self, bottom_fragment)
    }

    /// TODO: docs
    #[inline]
    pub fn split_y(mut self, split_at: Cells) -> (Self, Self) {
        let mut right_origin = self.origin;
        *right_origin.x_mut() += split_at;

        let mut right_size = self.size;
        *right_size.width_mut() -= split_at;

        *self.size.width_mut() = split_at;

        let right_fragment = Self {
            ptr: self.ptr,
            origin: right_origin,
            size: right_size,
            borrow: self.borrow.clone(),
            _lifetime: PhantomData,
        };

        (self, right_fragment)
    }

    /// TODO: docs
    #[inline]
    pub fn width(&self) -> Cells {
        self.size.width()
    }
}

/// TODO: docs.
pub trait Cutout {
    /// TODO: docs.
    type Cutout<'a>;

    /// TODO: docs.
    fn cutout(
        self,
        fragment: SceneFragment,
    ) -> (SceneFragment, Self::Cutout<'_>);
}
