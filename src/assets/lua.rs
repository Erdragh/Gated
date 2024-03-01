use std::string::FromUtf8Error;
use bevy::{
    asset::{AssetLoader, AsyncReadExt, io::Reader, LoadContext, ron},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use bevy::utils::thiserror;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LuaScript(pub String);

#[derive(Default)]
pub struct LuaScriptLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LuaScriptLoaderError {
    #[error("Could not load script: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to decode script: {0}")]
    Decode(#[from] FromUtf8Error),
}

impl AssetLoader for LuaScriptLoader {
    type Asset = LuaScript;
    type Settings = ();
    type Error = LuaScriptLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            info!("Loading Script {:?}", load_context.path());
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            Ok(LuaScript(String::from_utf8(bytes)?))
        })
    }
}