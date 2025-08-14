use core::cmp::Ordering;
use core::ops::RangeInclusive;
use std::collections::hash_map;

use collab_types::annotation::{
    AnnotationCreation,
    AnnotationDeletion,
    AnnotationId,
    AnnotationModification,
};
use collab_types::{Counter, PeerId, puff};
use fxhash::FxHashMap;
use nohash::IntMap as NoHashMap;
use puff::file::{GlobalFileId, LocalFileId};

use crate::fs::{Fs, PuffFile};

pub(crate) trait Annotation {
    type Op;
    type Backlog: Backlog<Annotation = Self>;
    type IntegrateResult;

    fn integrate_op(&mut self, op: Self::Op) -> Self::IntegrateResult;

    fn integrate_backlog(
        &mut self,
        backlog: Self::Backlog,
    ) -> Self::IntegrateResult;
}

pub(crate) trait Backlog {
    type Annotation: Annotation<Backlog = Self>;

    fn insert(&mut self, op: <Self::Annotation as Annotation>::Op);
    fn new(op: <Self::Annotation as Annotation>::Op) -> Self;
}

#[derive(cauchy::Debug, cauchy::Clone, cauchy::Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct Annotations<T: Annotation> {
    /// TODO: docs.
    alive: FxHashMap<AnnotationId, AnnotationData<T>>,

    /// TODO: docs.
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "ignore_and_zero")
    )]
    next_seq: Counter<u64>,

    /// TODO: docs.
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "T::Backlog: serde::Serialize",
            deserialize = "T::Backlog: serde::Deserialize<'de>",
        ))
    )]
    backlog: FxHashMap<AnnotationId, T::Backlog>,

    /// Creations that have been backlogged due to the local fs not yet having
    /// received the creation of the file they're in.
    backlogged_creations: FxHashMap<GlobalFileId, AnnotationCreation<T>>,

    /// Map from annotation owner to the ranges of sequences of annotations
    /// that have been deleted. The ranges are disjoint and ordered in
    /// ascending order.
    ///
    /// For example, if a peer has created 5 annotations and removed all but
    /// the 3rd, the ranges would be `[0..=1, 3..=4]`.
    ///
    /// Conversely, if the 3rd annotation was the only one removed, the ranges
    /// would be `[2..=2]`.
    deleted: NoHashMap<PeerId, Vec<RangeInclusive<u64>>>,
}

pub(crate) struct AnnotationRef<'a, T: Annotation> {
    data: &'a AnnotationData<T>,
    id: AnnotationId,
}

pub(crate) struct AnnotationMut<'a, T: Annotation> {
    annotations: &'a mut Annotations<T>,
    id: AnnotationId,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct AnnotationData<T> {
    inner: T,
    local_file_id: LocalFileId,
}

pub(crate) struct AnnotationsIter<'a, T: Annotation> {
    iters: hash_map::Iter<'a, AnnotationId, AnnotationData<T>>,
}

impl<T: Annotation> Annotations<T> {
    #[inline]
    pub(crate) fn create(
        &mut self,
        local_id: PeerId,
        in_file: PuffFile<'_, impl Sized>,
        data: T,
    ) -> (AnnotationMut<'_, T>, AnnotationCreation<T>)
    where
        T: Clone,
    {
        let annotation_id = self.next_id(local_id);

        let annotation_data = AnnotationData {
            inner: data.clone(),
            local_file_id: in_file.local_id(),
        };

        self.alive.insert(annotation_id, annotation_data);

        let creation = AnnotationCreation {
            annotation_id,
            data,
            file_id: in_file.global_id(),
        };

        (AnnotationMut { annotations: self, id: annotation_id }, creation)
    }

    #[inline]
    pub(crate) fn get(
        &self,
        annotation_id: AnnotationId,
    ) -> Option<AnnotationRef<'_, T>> {
        self.alive
            .get(&annotation_id)
            .map(|data| AnnotationRef { data, id: annotation_id })
    }

    #[inline]
    pub(crate) fn get_mut(
        &mut self,
        annotation_id: AnnotationId,
    ) -> Option<AnnotationMut<'_, T>> {
        self.alive
            .contains_key(&annotation_id)
            .then_some(AnnotationMut { annotations: self, id: annotation_id })
    }

    #[inline]
    pub(crate) fn integrate_creation(
        &mut self,
        mut creation: AnnotationCreation<T>,
        fs: &Fs,
    ) -> Option<AnnotationMut<'_, T>> {
        if self.is_deleted(creation.annotation_id) {
            None
        } else {
            if let Some(backlog) = self.backlog.remove(&creation.annotation_id)
            {
                creation.data.integrate_backlog(backlog);
            }

            let Some(local_id) =
                fs.local_file_id_of_global_id(creation.file_id)
            else {
                self.backlogged_creations.insert(creation.file_id, creation);
                return None;
            };

            let data = AnnotationData {
                inner: creation.data,
                local_file_id: local_id,
            };

            self.alive.insert(creation.annotation_id, data);

            Some(AnnotationMut {
                annotations: self,
                id: creation.annotation_id,
            })
        }
    }

    #[inline]
    pub(crate) fn integrate_deletion(
        &mut self,
        deletion: AnnotationDeletion,
    ) -> Option<(AnnotationId, LocalFileId)> {
        if self.is_deleted(deletion.annotation_id) {
            None
        } else {
            let data = self.delete(deletion.annotation_id)?;
            Some((deletion.annotation_id, data.local_file_id))
        }
    }

    #[inline]
    pub(crate) fn integrate_file_creation(
        &mut self,
        local_file_id: LocalFileId,
        global_id: GlobalFileId,
    ) {
        let Some(mut creation) = self.backlogged_creations.remove(&global_id)
        else {
            return;
        };

        if self.is_deleted(creation.annotation_id) {
            return;
        }

        if let Some(backlog) = self.backlog.remove(&creation.annotation_id) {
            creation.data.integrate_backlog(backlog);
        }

        self.alive.insert(
            creation.annotation_id,
            AnnotationData { inner: creation.data, local_file_id },
        );
    }

    #[inline]
    pub(crate) fn integrate_op(
        &mut self,
        op: AnnotationModification<T::Op>,
    ) -> Option<(AnnotationMut<'_, T>, T::IntegrateResult)> {
        if self.is_deleted(op.annotation_id) {
            None
        } else if let Some(data) = self.alive.get_mut(&op.annotation_id) {
            let res = data.inner.integrate_op(op.data);
            Some((
                AnnotationMut { annotations: self, id: op.annotation_id },
                res,
            ))
        } else {
            match self.backlog.entry(op.annotation_id) {
                hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(op.data);
                },
                hash_map::Entry::Vacant(entry) => {
                    entry.insert(T::Backlog::new(op.data));
                },
            }
            None
        }
    }

    #[inline]
    pub(crate) fn iter(&self) -> AnnotationsIter<'_, T> {
        AnnotationsIter { iters: self.alive.iter() }
    }

    #[inline]
    fn delete(&mut self, id: AnnotationId) -> Option<AnnotationData<T>> {
        debug_assert!(!self.is_deleted(id));
        self.backlog.remove(&id);
        self.mark_deleted(id);
        self.alive.remove(&id)
    }

    #[inline]
    fn is_deleted(&self, id: AnnotationId) -> bool {
        self.removed_range_idx_containing(id).is_ok()
    }

    #[inline]
    fn mark_deleted(&mut self, id: AnnotationId) {
        debug_assert!(!self.is_deleted(id));

        let insert_idx = self.removed_range_idx_containing(id).expect_err(
            "the annotation was alive, so it can't have already been deleted",
        );

        let peer_id = id.created_by;
        let seq = id.sequence_num;

        let Some(ranges) = self.deleted.get_mut(&peer_id) else {
            self.deleted.insert(peer_id, vec![seq..=seq]);
            return;
        };

        let is_one_more_than_prev_end =
            insert_idx > 0 && seq == *ranges[insert_idx - 1].end() + 1;

        let is_one_less_than_next_start = insert_idx < ranges.len()
            && *ranges[insert_idx].start() == seq + 1;

        match (is_one_more_than_prev_end, is_one_less_than_next_start) {
            (true, true) => {
                let next = ranges.remove(insert_idx);
                let prev = &mut ranges[insert_idx - 1];
                *prev = *prev.start()..=*next.end();
            },
            (true, false) => {
                let prev = &mut ranges[insert_idx - 1];
                *prev = *prev.start()..=seq;
            },
            (false, true) => {
                let next = &mut ranges[insert_idx];
                *next = seq..=*next.end();
            },
            (false, false) => {
                let new_range = seq..=seq;
                ranges.insert(insert_idx, new_range);
            },
        }
    }

    #[inline]
    fn next_id(&mut self, local_id: PeerId) -> AnnotationId {
        AnnotationId {
            created_by: local_id,
            sequence_num: self.next_seq.post_increment(),
        }
    }

    #[inline]
    fn removed_range_idx_containing(
        &self,
        id: AnnotationId,
    ) -> Result<usize, usize> {
        let Some(ranges) = self.deleted.get(&id.created_by) else {
            return Err(0);
        };

        ranges.binary_search_by(|range| {
            if id.sequence_num < *range.start() {
                Ordering::Greater
            } else if id.sequence_num > *range.end() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
    }
}

impl<'a, T: Annotation> AnnotationRef<'a, T> {
    #[inline]
    pub(crate) fn data(&self) -> &'a T {
        &self.data.inner
    }

    #[inline]
    pub(crate) fn file_id(&self) -> LocalFileId {
        self.data.local_file_id
    }

    #[inline]
    pub(crate) fn id(&self) -> AnnotationId {
        self.id
    }
}

impl<'a, T: Annotation> AnnotationMut<'a, T> {
    #[inline]
    pub(crate) fn data_mut(&mut self) -> &mut T {
        &mut self.data_mut_inner().inner
    }

    #[inline]
    pub(crate) fn delete(self) -> AnnotationDeletion {
        self.annotations.delete(self.id);
        AnnotationDeletion { annotation_id: self.id }
    }

    #[inline]
    pub(crate) fn id(&self) -> AnnotationId {
        self.id
    }

    #[inline]
    pub(crate) fn update(
        &mut self,
        updater: impl FnOnce(&mut T) -> T::Op,
    ) -> AnnotationModification<T::Op> {
        AnnotationModification {
            annotation_id: self.id,
            data: updater(self.data_mut()),
        }
    }

    #[inline]
    fn data_mut_inner(&mut self) -> &mut AnnotationData<T> {
        self.annotations.alive.get_mut(&self.id).expect("ID is valid")
    }
}

impl<'a, T: Annotation> Iterator for AnnotationsIter<'a, T> {
    type Item = AnnotationRef<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iters.next().map(|(id, data)| AnnotationRef { data, id: *id })
    }
}

#[cfg(feature = "serde")]
fn ignore_and_zero<'de, D>(de: D) -> Result<Counter<u64>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::Deserialize;
    let _ = Counter::<u64>::deserialize(de)?;
    Ok(Counter::new(0))
}
