use std::sync::Arc;

use flume::{Receiver, Sender};

use crate::AuthInfos;

#[derive(Clone)]
pub(crate) struct CredentialStore {
    /// TODO: remove this once https://github.com/zesterer/flume/issues/155 is
    /// addressed.
    rx: Arc<Receiver<Request>>,
    tx: Sender<Request>,
}

pub(crate) enum Error {
    GetCredential(keyring::Error),
    Op(keyring::Error),
}

enum Request {
    Delete(Sender<Result<(), Error>>),
    Persist(AuthInfos, Sender<Result<(), Error>>),
    Retrieve(Sender<Result<Option<AuthInfos>, Error>>),
}

impl CredentialStore {
    const APP_NAME: &str = "nomad";
    const SECRET_NAME: &str = "auth-infos";

    pub(crate) async fn delete(&self) -> Result<(), Error> {
        let (tx, rx) = flume::bounded(1);
        self.send_request(Request::Delete(tx)).await;
        rx.recv_async().await.expect("tx is still alive")
    }

    pub(crate) async fn persist(&self, infos: AuthInfos) -> Result<(), Error> {
        let (tx, rx) = flume::bounded(1);
        self.send_request(Request::Persist(infos, tx)).await;
        rx.recv_async().await.expect("tx is still alive")
    }

    pub(crate) async fn retrieve(&self) -> Result<Option<AuthInfos>, Error> {
        let (tx, rx) = flume::bounded(1);
        self.send_request(Request::Retrieve(tx)).await;
        rx.recv_async().await.expect("tx is still alive")
    }

    pub(crate) async fn run(self, builder: Box<keyring::CredentialBuilder>) {
        let mut maybe_entry = None;

        while let Ok(req) = self.rx.recv() {
            let entry = match &maybe_entry {
                Some(entry) => entry,
                None => match builder.build(
                    None,
                    Self::APP_NAME,
                    Self::SECRET_NAME,
                ) {
                    Ok(cred) => {
                        let entry = keyring::Entry::new_with_credential(cred);
                        &*maybe_entry.insert(entry)
                    },
                    Err(err) => {
                        req.query_err(err);
                        continue;
                    },
                },
            };

            match req {
                Request::Delete(tx) => {
                    let msg = entry.delete_credential().map_err(Error::Op);
                    let _ = tx.send(msg);
                },
                Request::Persist(infos, tx) => {
                    let json =
                        serde_json::to_string(&infos).expect("never fails");
                    let msg = entry.set_password(&json).map_err(Error::Op);
                    let _ = tx.send(msg);
                },
                Request::Retrieve(tx) => {
                    let msg = entry
                        .get_password()
                        .map(|json| serde_json::from_str(&json).ok())
                        .map_err(Error::Op);
                    let _ = tx.send(msg);
                },
            }
        }
    }

    async fn send_request(&self, req: Request) {
        self.tx
            .send_async(req)
            .await
            .expect("we have an instance of the Receiver, this can't fail")
    }
}

impl Request {
    fn query_err(&self, err: keyring::Error) {
        match self {
            Request::Delete(tx) => {
                let _ = tx.send(Err(Error::GetCredential(err)));
            },
            Request::Persist(_, tx) => {
                let _ = tx.send(Err(Error::GetCredential(err)));
            },
            Request::Retrieve(tx) => {
                let _ = tx.send(Err(Error::GetCredential(err)));
            },
        }
    }
}

impl Default for CredentialStore {
    fn default() -> Self {
        let (tx, rx) = flume::bounded(1);
        Self { rx: Arc::new(rx), tx }
    }
}
