use std::sync::Arc;

use super::html::Tag;
use mlua::prelude::*;

// use tokio::time::{sleep, Duration};

// async fn sleep_ms(_lua: &Lua, ms: u64) -> LuaResult<()> {
//     sleep(Duration::from_millis(ms)).await;
//     Ok(())
// }

fn get(lua: &'static Lua, class: String, tags: &'static Vec<Tag>) -> LuaResult<LuaTable<'static>> {
    for tag in tags {
        if tag.classes.contains(&class) {
            let table = lua.create_table()?;

            table.set("name", tag.widget.get_css_name())?;

            table.set(
                "get_content",
                lua.create_function(|_, ()| Ok(tag.widget.get_contents()))?,
            )?;
            table.set(
                "set_content",
                lua.create_function(|_, label: String| {
                    tag.widget._set_label(label);
                    Ok(())
                })?,
            )?;
            table.set(
                "on_click",
                lua.create_function(move |_lua, func: LuaFunction| {
                    tag.widget._on_click(&func);
                    Ok(())
                })?,
            )?;

            return Ok(table);
        }
    }

    Err(LuaError::RuntimeError("Tag not found".into()))
}

fn print(_lua: &Lua, msg: String) -> LuaResult<()> {
    println!("{}", msg);
    Ok(())
}

pub(crate) fn run(tags: Vec<Tag>) -> LuaResult<()> {
    let lua = Lua::new();
    let globals = lua.globals();

    let tags_static: &'static Vec<Tag> = Box::leak(Box::new(tags));

    // globals.set("sleep_ms", lua.create_async_function(sleep_ms)?)?;
    globals.set("print", lua.create_function(print)?)?;
    globals.set(
        "get",
        lua.create_function(move |lua, class: String| get(lua, class, tags_static))?,
    )?;

    let ok = lua
        .load(include_str!("../../test/script.lua"))
        .eval::<LuaMultiValue>();

    match ok {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(
                "--------------------------\nerror: {}\n--------------------------------",
                e
            );
            Err(LuaError::runtime("Failed to run script!"))
        }
    }
}
