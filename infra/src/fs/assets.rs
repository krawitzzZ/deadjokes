use dropbox_sdk::{
    default_client::UserAuthDefaultClient, files, oauth2::get_auth_from_env_or_prompt,
};
use futures_util::TryFutureExt;
use std::{
    io::{self, Read},
    path::PathBuf,
};
use tokio::fs as async_fs;

use app::{AppError, AppResult, Assets};

const JOKES_INITIAL_SEED_SQL_FILE: &str = ".data/jokes.sql";

impl FsAssetsParameters {
    pub fn new() -> AppResult<Self> {
        let pwd = std::env::current_dir().map_err(|e| {
            AppError::AssetError(format!("failed to get jokes seed file path: {e}"))
        })?;
        let jokes_seed_sql = pwd.join(JOKES_INITIAL_SEED_SQL_FILE);

        Ok(Self { jokes_seed_sql })
    }
}

#[derive(Debug, Clone, shaku::Component)]
#[shaku(interface = Assets)]
pub struct FsAssets {
    jokes_seed_sql: PathBuf,
}

impl FsAssets {
    pub fn new(params: &FsAssetsParameters) -> Self {
        Self {
            jokes_seed_sql: params.jokes_seed_sql.clone(),
        }
    }

    fn download_seed_sql(&self) -> io::Result<()> {
        let dbx_auth = get_auth_from_env_or_prompt();
        let client = UserAuthDefaultClient::new(dbx_auth);
        let download_arg = files::DownloadArg::new("/jokes.sql".to_owned());
        let mut file = std::fs::File::create(&self.jokes_seed_sql)?;
        let mut bytes_out = 0u64;

        'download: loop {
            match files::download(&client, &download_arg, Some(bytes_out), None) {
                Ok(Ok(download_result)) => {
                    let mut body = download_result.body.ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::ConnectionReset,
                            "download jokes seed error: no body received",
                        )
                    })?;
                    loop {
                        let mut input_chunk = (&mut body).take(1024 * 1024 * 10);
                        match io::copy(&mut input_chunk, &mut file) {
                            Ok(0) => {
                                return Ok(());
                            }
                            Ok(len) => {
                                bytes_out += len;
                            }
                            Err(_) => {
                                continue 'download; // do another request and resume
                            }
                        }
                    }
                }
                Ok(Err(download_error)) => {
                    return Err(io::Error::new(
                        io::ErrorKind::ConnectionReset,
                        format!("download jokes seed error: {download_error}"),
                    ));
                }
                Err(request_error) => {
                    return Err(io::Error::new(
                        io::ErrorKind::ConnectionReset,
                        format!("download jokes seed error: {request_error}"),
                    ));
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl Assets for FsAssets {
    async fn initial_jokes_seed(&self) -> AppResult<String> {
        let seed_file_exists = &self.jokes_seed_sql.try_exists().map_err(|e| {
            AppError::AssetError(format!("failed to check jokes seed file existence: {e}"))
        })?;

        if !seed_file_exists {
            self.download_seed_sql()
                .map_err(|e| AppError::AssetError(format!("failed to download jokes seed: {e}")))?;
        }

        async_fs::read_to_string(&self.jokes_seed_sql)
            .map_err(|e| AppError::AssetError(format!("failed to read jokes seed: {e}")))
            .await
    }
}