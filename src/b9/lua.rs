use std::cell::RefCell;
use std::rc::Rc;

use super::html::Tag;
use mlua::prelude::*;

// use tokio::time::{sleep, Duration};

// async fn sleep_ms(_lua: &Lua, ms: u64) -> LuaResult<()> {
//     sleep(Duration::from_millis(ms)).await;
//     Ok(())
// }

fn get<'lua>(lua: &'lua Lua, class: String, tags: Rc<RefCell<Vec<Tag>>>) -> LuaResult<LuaTable<'lua>> {
    let tags_ref = tags.borrow();

    for (i, tag) in tags_ref.iter().enumerate() {
        if tag.classes.contains(&class) {
            let tags1 = Rc::clone(&tags);
            let tags2= Rc::clone(&tags);
            let tags3 = Rc::clone(&tags);

            let table = lua.create_table()?;

            let css_name = tag.widget.get_css_name().clone();

            table.set("name", css_name)?;

            table.set(
                "get_content",
                lua.create_function(move |_, ()| {
                    let ok = tags1.borrow()[i].widget.get_contents().clone();
                    Ok(ok)  
                })?,
            )?;
            table.set(
                "set_content",
                lua.create_function(move |_, label: String| {
                    tags2.borrow()[i].widget._set_label(label.clone());
                    Ok(())
                })?,
            )?;
            table.set(
                "on_click",
                lua.create_function(move |_lua, func: LuaFunction| {
                    tags3.borrow()[i].widget._on_click(Rc::new(RefCell::new(func)));
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

pub(crate) fn run(tags: Rc<RefCell<Vec<Tag>>>) -> LuaResult<()> {
    let lua = Lua::new();
    let globals = lua.globals();

    // globals.set("sleep_ms", lua.create_async_function(sleep_ms)?)?;
    globals.set("print", lua.create_function(print)?)?;
    globals.set(
        "get",
        lua.create_function(move |lua, class: String| get(lua, class, tags))?,
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
