use mlua::prelude::*;

pub fn test_lua() -> LuaResult<()> {
    let lua = Lua::new();

    lua.load("print(\"Hello World!\")").exec()?;

    Ok(())
}
