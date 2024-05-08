use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use super::css::Styleable;
use super::html::Tag;
use mlua::prelude::*;
use mlua::OwnedFunction;
use gtk::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref LUA_LOGS: Mutex<String> = Mutex::new(String::new());
}
macro_rules! problem {
    ($type:expr, $s:expr) => {{
        let problem_type = match ($type) {
            "error" => "ERROR",
            "warning" => "WARNING",
            _ => "UNKNOWN",
        };

        let log_msg = format!("{}: {}\n", problem_type, $s);

        if let Ok(mut lua_logs) = LUA_LOGS.lock() {
            lua_logs.push_str(&log_msg);
        } else {
            eprintln!("FATAL: failed to lock lua logs mutex!");
        }
    }};
}

pub trait Luable: Styleable {
    fn get_children(&self) -> Vec<gtk::Widget>;
    fn get_css_classes(&self) -> Vec<String>;
    fn get_css_name(&self) -> String;
    fn get_contents(&self) -> String;
    fn _set_label(&self, contents: String);
    fn _on_click<'a>(&self, func: &'a LuaOwnedFunction);
    fn _on_submit<'a>(&self, func: &'a LuaOwnedFunction);
    fn _on_input<'a>(&self, func: &'a LuaOwnedFunction);
}
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
            let tags4 = Rc::clone(&tags);
            let tags5 = Rc::clone(&tags);

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
                lua.create_function(move |_lua, func: OwnedFunction| {
                    tags3.borrow()[i].widget._on_click(&func);
                    Ok(())
                })?,
            )?;
            table.set(
                "on_submit",
                lua.create_function(move |_lua, func: OwnedFunction| {
                    tags4.borrow()[i].widget._on_submit(&func);
                    Ok(())
                })?,
            )?;
            table.set(
                "on_input",
                lua.create_function(move |_lua, func: OwnedFunction| {
                    tags5.borrow()[i].widget._on_input(&func);
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
        lua.create_function(move |lua, class: String| get(lua, class, tags.clone()))?,
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

// h1, h2, h3, h4, h5, h6, p
impl Luable for gtk::Label {
    fn get_children(&self) -> Vec<gtk::Widget> {
        Vec::new()
    }

    fn get_css_classes(&self) -> Vec<String> {
        self.css_classes().iter().map(|s| s.to_string()).collect()
    }

    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents(&self) -> String {
        self.text().to_string()
    }

    fn _set_label(&self, contents: String) {
        self.set_text(&contents);
    }

    fn _on_click<'a>(&self, func: &'a LuaOwnedFunction) {
        let gesture = gtk::GestureClick::new();

        let a = Rc::new(func.clone());

        gesture.connect_released(move |_, _, _, _| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                problem!("error", e.to_string());
            }
        });

        self.add_controller(gesture)
    }
    fn _on_submit<'a>(&self, _: &'a LuaOwnedFunction) {
        problem!(
            "warning",
            "Text-based components do not support the \"submit\" event. Are you perhaps looking for the \"click\" event?"
        );
    }
    fn _on_input<'a>(&self, _: &'a LuaOwnedFunction) {
        problem!(
            "warning",
            "Text-based components do not support the \"input\" event."
        );
    }
}