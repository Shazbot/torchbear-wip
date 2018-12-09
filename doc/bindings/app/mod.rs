use rlua::prelude::*;

pub mod git;
pub mod log;
pub mod markdown;
pub mod tera;

#[cfg(feature = "tantivy_bindings")]
pub mod tantivy;

// Dummy modules
#[cfg(not(feature = "tantivy_bindings"))]
pub mod tantivy {
    pub fn init(_: &rlua::Lua) -> rlua::Result<()> { Ok(()) }
}

pub fn init(lua: &Lua) -> LuaResult<()> {
    git::init(&lua)?;
    log::init(&lua)?;
    markdown::init(&lua)?;
    tantivy::init(&lua)?;
    tera::init(&lua)?;

    Ok(())
}