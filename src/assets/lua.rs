use std::borrow::Cow;
use std::ffi::OsString;
use std::string::FromUtf8Error;
use bevy::{
    asset::{AssetLoader, AsyncReadExt, io::Reader, LoadContext, ron},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use bevy::utils::thiserror;
use mlua::{AsChunk, ChunkMode, Lua, Table};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LuaScript {
    pub script: String,
    pub name: OsString
}

impl<'a> AsChunk<'_, 'a> for &'a LuaScript {
    fn name(&self) -> Option<String> {
        self.name.to_string_lossy().get(0..).map(str::to_string)
    }
    fn mode(&self) -> Option<ChunkMode> {
        Some(ChunkMode::Text)
    }
    fn source(self) -> std::io::Result<Cow<'a, [u8]>> {
        Ok(Cow::Borrowed((&self.script).as_ref()))
    }
}

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
            Ok(LuaScript {
                script: String::from_utf8(bytes) ?,
                name: load_context.path().as_os_str().to_os_string()
            })
        })
    }
}