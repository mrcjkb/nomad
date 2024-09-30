use collab_server::SessionId;
use futures_util::{select, FutureExt};
use nomad2::neovim::{self, Neovim, NeovimModuleApi};
use nomad2::{
    module_name,
    Api,
    Context,
    Editor,
    Event,
    JoinHandle,
    Module,
    ModuleName,
    Spawner,
    Subscription,
};

use crate::events::{JoinSession, StartSession};
use crate::{Config, Session};

/// TODO: docs.
pub struct Collab<E: Editor> {
    pub(crate) ctx: Context<E>,
    pub(crate) config: Config,
    pub(crate) join_sub: Subscription<JoinSession, E>,
    pub(crate) start_sub: Subscription<StartSession, E>,
}

impl<E: Editor> Collab<E>
where
    JoinSession: Event<E, Payload = SessionId>,
    StartSession: Event<E>,
{
    const NAME: ModuleName = module_name!("collab");

    fn join_session(&self, id: SessionId) {
        let ctx = self.ctx.clone();
        let config = self.config.clone();

        let fut = async move {
            let session = match Session::join(id, config, ctx).await {
                Ok(session) => session,
                Err(err) => {
                    println!("{err:?}");
                    return;
                },
            };

            if let Err(err) = session.run().await {
                println!("{err:?}");
            }
        };

        self.ctx.spawner().spawn(fut).detach();
    }

    async fn run(&mut self) {
        loop {
            select! {
                _ = self.start_sub.recv().fuse() => self.start_session(),
                &id = self.join_sub.recv().fuse() => self.join_session(id),
            }
        }
    }

    fn start_session(&self) {
        let ctx = self.ctx.clone();
        let config = self.config.clone();

        let fut = async move {
            let session = match Session::start(config, ctx).await {
                Ok(session) => session,
                Err(err) => {
                    println!("{err:?}");
                    return;
                },
            };

            if let Err(err) = session.run().await {
                println!("{err:?}");
            }
        };

        self.ctx.spawner().spawn(fut).detach();
    }
}

impl Module<Neovim> for Collab<Neovim> {
    const NAME: ModuleName = Self::NAME;

    type Config = Config;

    fn api(ctx: &Context<Neovim>) -> NeovimModuleApi<Self> {
        // let (join_fn, join_fn_sub) = NeovimFunction::builder()
        //     .name(JoinSession::NAME)
        //     .args::<SessionId>()
        //     .build::<Self>(ctx);
        //
        // let (start_fn, start_fn_sub) = NeovimFunction::builder()
        //     .name(StartSession::NAME)
        //     .args::<()>()
        //     .build::<Self>(ctx);

        let (join_cmd, join_cmd_sub) = NeovimCommand::new(JoinSession, ctx);
        let (start_cmd, start_cmd_sub) = NeovimCommand::new(StartSession, ctx);

        let (join_fn, join_fn_sub) = neovim::function::<JoinSession>(ctx);
        let (start_fn, start_fn_sub) = neovim::function::<StartSession>(ctx);

        let this = Self(Collab {
            ctx: ctx.clone(),
            config: Config::default(),
            join_sub: join_cmd_sub.zip(join_fn_sub),
            start_sub: start_cmd_sub.zip(start_fn_sub),
        });

        NeovimModuleApi::new(this)
            // .with_default_command(Auth)
            .with_command(join_cmd)
            .with_command(start_cmd)
            .with_function(join_fn)
            .with_function(start_fn)
    }

    async fn run(&mut self, _: &Context<Neovim>) {
        self.run().await;
    }
}
