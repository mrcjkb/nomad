use core::any::type_name;

use collab_server::message::Message;
use nomad::events::{Cursor, CursorAction};
use nomad::{action_name, Action, ActionName, Shared, ShouldDetach};

use super::Project;
use crate::Collab;

#[derive(Clone)]
pub(super) struct SyncCursor {
    pub(super) message_tx: flume::Sender<Message>,
    pub(super) project: Shared<Project>,
    pub(super) should_detach: Shared<ShouldDetach>,
}

impl Action for SyncCursor {
    const NAME: ActionName = action_name!("synchronize-cursor");
    type Args = Cursor;
    type Docs = ();
    type Module = Collab;
    type Return = ShouldDetach;

    fn execute(&mut self, cursor: Self::Args) -> Self::Return {
        let message = self.project.with_mut(|project| {
            if cursor.moved_by == project.actor_id {
                return None;
            }

            let Some(mut file) =
                project.file_mut_of_buffer_id(cursor.buffer_id)
            else {
                unreachable!(
                    "couldn't convert BufferId to file in {}",
                    type_name::<Self>()
                );
            };

            Some(match cursor.action {
                CursorAction::Created(byte_offset) => {
                    let (cursor_id, cursor_creation) =
                        file.sync_created_cursor(byte_offset.into_u64());
                    assert!(
                        project.local_cursor_id.is_none(),
                        "creating a new cursor when another already exists, \
                         but Neovim only supports a single cursor"
                    );
                    project.local_cursor_id = Some(cursor_id);
                    todo!();
                },
                CursorAction::Moved(byte_offset) => {
                    project
                        .local_cursor_mut()
                        .expect("cursor is being moved, so it must exist")
                        .sync_relocated(byte_offset.into_u64());
                    todo!();
                },
                CursorAction::Removed => {
                    project
                        .local_cursor_mut()
                        .expect("cursor is being removed, so it must exist")
                        .sync_removed();
                    todo!();
                },
            })
        });

        if let Some(message) = message {
            if self.message_tx.send(message).is_err() {
                self.should_detach.set(ShouldDetach::Yes);
            }
        }

        self.should_detach.get()
    }

    fn docs(&self) {}
}
