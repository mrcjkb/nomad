use crate::AuthInfos;

#[derive(Default, Clone)]
pub(crate) struct CredentialStore {}

#[derive(Default)]
pub(crate) struct CredentialEntry {}

impl CredentialStore {
    const APP_NAME: &str = "nomad";
    const SECRET_NAME: &str = "auth-infos";

    pub(crate) async fn get_entry(
        &self,
    ) -> Result<CredentialEntry, keyring::Error> {
        todo!()
    }

    pub(crate) async fn run(
        self,
        builder: Box<keyring::CredentialBuilder>,
    ) -> ! {
        loop {
            let _entry_entry =
                match builder.build(None, Self::APP_NAME, Self::SECRET_NAME) {
                    Ok(credential) => {
                        keyring::Entry::new_with_credential(credential)
                    },
                    Err(_err) => todo!(),
                };

            todo!()
        }
    }
}

impl CredentialEntry {
    pub(crate) async fn persist(
        &self,
        _auth_infos: AuthInfos,
    ) -> Result<(), keyring::Error> {
        todo!()
    }
}
