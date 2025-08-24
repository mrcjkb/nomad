use core::fmt::Display;

use editor::context::BorrowState;
use editor::{AccessMut, AgentId, Buffer, Context, Editor};
use futures_util::FutureExt;

pub(crate) trait ContextExt<Ed: Editor> {
    fn create_and_focus(
        &mut self,
        file_path: &abs_path::AbsPath,
        agent_id: AgentId,
    ) -> impl Future<Output = Ed::BufferId>
    where
        Ed::CreateBufferError: Display;

    fn create_scratch_buffer(
        &mut self,
        agent_id: AgentId,
    ) -> impl Future<Output = Ed::BufferId>
    where
        Ed: TestEditor;

    fn create_and_focus_scratch_buffer(
        &mut self,
        agent_id: AgentId,
    ) -> impl Future<Output = Ed::BufferId>
    where
        Ed: TestEditor;
}

pub(crate) trait TestEditor: Editor {
    fn create_scratch_buffer(
        this: impl AccessMut<Self>,
        agent_id: AgentId,
    ) -> impl Future<Output = Self::BufferId>;
}

#[cfg(feature = "neovim")]
impl TestEditor for neovim::Neovim {
    async fn create_scratch_buffer(
        mut this: impl AccessMut<Self>,
        agent_id: AgentId,
    ) -> Self::BufferId {
        use neovim::oxi::api::{self, opts};
        use neovim::tests::NeovimExt;

        let buf_id = this.create_scratch_buffer(agent_id);

        // The (fix)eol options mess us the fuzzy edits tests because inserting
        // text when the buffer is empty will also cause a trailing \n to be
        // inserted, so unset them.
        let opts = opts::OptionOpts::builder().buffer(buf_id.into()).build();
        api::set_option_value::<bool>("eol", false, &opts).unwrap();
        api::set_option_value::<bool>("fixeol", false, &opts).unwrap();

        buf_id
    }
}

impl TestEditor for mock::Mock {
    async fn create_scratch_buffer(
        mut this: impl AccessMut<Self>,
        agent_id: AgentId,
    ) -> Self::BufferId {
        let scratch_file_path = |num_scratch: u32| {
            let file_name = format!("scratch-{num_scratch}")
                .parse::<abs_path::NodeNameBuf>()
                .expect("it's valid");
            abs_path::AbsPathBuf::root().join(&file_name)
        };

        let mut num_scratch = 0;

        loop {
            let file_path = scratch_file_path(num_scratch);
            if this.with_mut(|mock| mock.buffer_at_path(&file_path).is_none())
            {
                return Self::create_buffer(this, &file_path, agent_id)
                    .await
                    .expect("couldn't create buffer");
            }
            num_scratch += 1;
        }
    }
}

impl<Ed: Editor, Bs: BorrowState> ContextExt<Ed> for Context<Ed, Bs>
where
    Self: AccessMut<Ed>,
{
    async fn create_and_focus(
        &mut self,
        file_path: &abs_path::AbsPath,
        agent_id: AgentId,
    ) -> Ed::BufferId
    where
        Ed::CreateBufferError: Display,
    {
        match self.create_buffer(file_path, agent_id).await {
            Ok(buffer_id) => {
                focus_buffer::<Ed>(self, buffer_id.clone(), agent_id).await;
                buffer_id
            },
            Err(err) => {
                panic!("couldn't create buffer at {file_path}: {err}")
            },
        }
    }

    async fn create_scratch_buffer(
        &mut self,
        agent_id: AgentId,
    ) -> Ed::BufferId
    where
        Ed: TestEditor,
    {
        Ed::create_scratch_buffer(self, agent_id).await
    }

    async fn create_and_focus_scratch_buffer(
        &mut self,
        agent_id: AgentId,
    ) -> Ed::BufferId
    where
        Ed: TestEditor,
    {
        let buffer_id = self.create_scratch_buffer(agent_id).await;
        focus_buffer::<Ed>(self, buffer_id.clone(), agent_id).await;
        buffer_id
    }
}

async fn focus_buffer<Ed: Editor>(
    mut ctx: impl AccessMut<Ed>,
    buffer_id: Ed::BufferId,
    agent_id: AgentId,
) {
    ctx.with_mut(|ed| {
        ed.buffer(buffer_id.clone())
            .expect("invalid buffer ID")
            .schedule_focus(agent_id)
            .boxed_local()
    })
    .await
}
