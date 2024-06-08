use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

use super::css::Styleable;
use super::html::Tag;
use glib::GString;
use gtk::prelude::*;
use gtk::CssProvider;
use mlua::{prelude::*, StdLib};

use mlua::{OwnedFunction, Value};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Map;

use crate::{globals::LUA_TIMEOUTS, lualog, Tab};
use glib::translate::FromGlib;
use glib::SourceId;

// from https://crates.io/crates/clone-macro
macro_rules! clone {
    () => {};
    ([$($tt:tt)*], $expr:expr) => {{
        clone!($($tt)*);
        $expr
    }};
    ($(,)? mut { $expr:expr } as $ident:ident $($tt:tt)*) => {
        let mut $ident = ::core::clone::Clone::clone(&$expr);
        clone!($($tt)*)
    };
    ($(,)? mut $ident:ident $($tt:tt)*) => {
        let mut $ident = ::core::clone::Clone::clone(&$ident);
        clone!($($tt)*)
    };
    ($(,)? { $expr:expr } as $ident:ident $($tt:tt)*) => {
        let $ident = ::core::clone::Clone::clone(&$expr);
        clone!($($tt)*)
    };
    ($(,)? $ident:ident $($tt:tt)*) => {
        let $ident = ::core::clone::Clone::clone(&$ident);
        clone!($($tt)*)
    };
    ($(,)?) => {};
}

pub trait Luable: Styleable {
    fn get_css_name(&self) -> String;

    fn get_contents_(&self) -> String {
        lualog!("warning", format!("get_content is not supported for {}", self.get_css_name()));
        String::new()
    }
    fn get_href_(&self) -> String {
        lualog!("warning", format!("get_href is not supported for {}", self.get_css_name()));
        String::new()
    }
    fn get_opacity_(&self) -> f64 {
        lualog!("warning", format!("get_opacity is not supported for {}", self.get_css_name()));
        1.0
    }
    fn get_source_(&self) -> String {
        lualog!("warning", format!("get_source is not supported for {}", self.get_css_name()));
        String::new()
    }
    fn set_contents_(&self, _contents: String) {
        lualog!("warning", format!("set_content is not supported for {}", self.get_css_name()));
    }
    fn set_href_(&self, _href: String) {
        lualog!("warning", format!("set_href is not supported for {}", self.get_css_name()));
    }
    fn set_opacity_(&self, _amount: f64) {
        lualog!("warning", format!("set_opacity is not supported for {}", self.get_css_name()));
    }
    fn set_source_(&self, _source: String) {
        lualog!("warning", format!("set_source is not supported for {}", self.get_css_name()));
    }
    fn set_visible_(&self, _visible: bool) {
        lualog!("warning", format!("set_visible is not supported for {}", self.get_css_name()));
    }
    fn set_inner_html(
        &self,
        _html: String,
        _scroll: Rc<RefCell<gtk::ScrolledWindow>>,
        _previous_css_provider: Option<CssProvider>,
        _searchbar: Rc<RefCell<gtk::SearchEntry>>,
        _current_tab: Rc<RefCell<Tab>>,
    ) -> LuaResult<Rc<RefCell<Vec<Tag>>>> {
        lualog!("warning", format!("set_inner_html is not supported for {}", self.get_css_name()));
        Err(LuaError::runtime("not supported"))
    }
    fn append_html(
        &self,
        _html: String,
        _scroll: Rc<RefCell<gtk::ScrolledWindow>>,
        _previous_css_provider: Option<CssProvider>,
        _searchbar: Rc<RefCell<gtk::SearchEntry>>,
        _current_tab: Rc<RefCell<Tab>>,
    ) -> LuaResult<Rc<RefCell<Vec<Tag>>>> {
        lualog!("warning", format!("append_html is not supported for {}", self.get_css_name()));
        Err(LuaError::runtime("not supported"))
    }

    fn _on_click(&self, _func: &LuaOwnedFunction) {
        lualog!("warning", format!("on_click is not supported for {}", self.get_css_name()));
    }
    fn _on_submit(&self, _func: &LuaOwnedFunction) {
        lualog!("warning", format!("on_submit is not supported for {}", self.get_css_name()));
    }
    fn _on_input(&self, _func: &LuaOwnedFunction) {
        lualog!("warning", format!("on_input is not supported for {}", self.get_css_name()));
    }
}

fn set_timeout(_lua: &Lua, func: LuaOwnedFunction, ms: u64) -> LuaResult<i32> {
    if let Ok(mut timeouts) = LUA_TIMEOUTS.lock() {
        if ms == 0 {
            if let Err(e) = func.call::<_, ()>(()) {
                lualog!(
                    "error",
                    format!("error calling function in set_timeout: {}", e)
                );
            }
            return Ok(-1);
        } else {
            let handle = glib::spawn_future_local(async move {
                glib::timeout_future(std::time::Duration::from_millis(ms)).await;
                if let Err(e) = func.call::<_, ()>(()) {
                    lualog!(
                        "error",
                        format!("error calling function in set_timeout: {}", e)
                    );
                }
            });
            timeouts.push(handle.source().clone());
            if let Some(id) = handle.as_raw_source_id() {
                return Ok(id as i32);
            } else {
                return Ok(-1);
            }
        }
    }
    Err(LuaError::runtime("couldn't create timeout"))
}

pub(crate) fn clear_timeout(id: i32) -> LuaResult<()> {
    if id > 0 {
        let id = unsafe { SourceId::from_glib(id.try_into().unwrap()) };
        if let Some(source) = glib::MainContext::default().find_source_by_id(&id) {
            source.destroy();
        }
    }
    Ok(())
}

fn modify_inner_html(
    html: String,
    widget: &gtk::Box,
    scroll: &Rc<RefCell<gtk::ScrolledWindow>>,
    previous_css_provider: &Option<CssProvider>,
    searchbar: &Rc<RefCell<gtk::SearchEntry>>,
    current_tab: &Rc<RefCell<Tab>>,
    append: bool
) -> LuaResult<Rc<RefCell<Vec<Tag>>>> {
    if !append {
        while let Some(el) = widget.last_child() {
            widget.remove(&el);
        }
    }

    let tags: Rc<RefCell<Vec<Tag>>> = Rc::new(RefCell::new(Vec::new()));
    let mut css = String::new();

    let dom = match html_parser::Dom::parse(&html) {
        Ok(dom) => dom,
        Err(_) => {
            lualog!(
                "error",
                "Invalid HTML"
            );
            return Err(LuaError::runtime("invalid html"));
        }
    };

    for element in dom.children.iter() {
        if let Some(element) = element.element() {
             println!("{:#?}", element);

             crate::b9::html::render_html(
                 &element,
                 element.children.first(),
                 widget.clone(),
                 true,
                 Rc::clone(&tags),
                 &mut css,
                 Rc::clone(&scroll),
                 previous_css_provider.clone(),
                 Rc::clone(&searchbar),
                 Rc::clone(&current_tab)
             );
         }
    }

    css.push_str(&widget.style());
    let _ = crate::css::load_css_into_app(&css);

    Ok(tags)
}

fn get(
    lua: &Lua, 
    class: String, 
    tags: Rc<RefCell<Vec<Tag>>>, 
    scroll: Rc<RefCell<gtk::ScrolledWindow>>,
    previous_css_provider: Option<CssProvider>,
    searchbar: Rc<RefCell<gtk::SearchEntry>>,
    current_tab: Rc<RefCell<Tab>>,
    multi: bool
) -> LuaResult<LuaTable> {
    let global_table = lua.create_table()?;
    let tags_ref = tags.borrow();

    for (i, tag) in tags_ref.iter().enumerate() {
        if tag.classes.contains(&class) {
            let table = lua.create_table()?;
            let css_name = tag.widget.get_css_name().clone();

            table.set("name", css_name)?;

            table.set(
                "get_content", 
                lua.create_function(clone!([tags], move |_, ()| {
                    let ok = tags.borrow()[i].widget.get_contents_();
                    Ok(ok)
                }))?
            )?;
            table.set(
                "set_content",
                lua.create_function(clone!([tags], move |_, label: Option<String>| {
                    let label = if let Some(label) = label {
                        label
                    } else {
                        "".to_string()
                    };
                    tags.borrow()[i].widget.set_contents_(label);
                    Ok(())
                }))?
            )?;
            table.set(
                "on_click",
                lua.create_function(clone!([tags], move |_lua, func: OwnedFunction| {
                    tags.borrow()[i].widget._on_click(&func);
                    Ok(())
                }))?
            )?;
            table.set(
                "on_submit",
                lua.create_function(clone!([tags], move |_lua, func: OwnedFunction| {
                    tags.borrow()[i].widget._on_submit(&func);
                    Ok(())
                }))?
            )?;
            table.set(
                "on_input",
                lua.create_function(clone!([tags], move |_lua, func: OwnedFunction| {
                    tags.borrow()[i].widget._on_input(&func);
                    Ok(())
                }))?
            )?;
            table.set(
                "get_href",
                lua.create_function(clone!([tags], move |_, ()| {
                    let ok = tags.borrow()[i].widget.get_href_();
                    Ok(ok)
                }))?
            )?;
            table.set(
                "set_href",
                lua.create_function(clone!([tags], move |_, label: String| {
                    tags.borrow()[i].widget.set_href_(label);
                    Ok(())
                }))?
            )?;
            table.set(
                "get_opacity",
                lua.create_function(clone!([tags], move |_, ()| {
                    let ok = tags.borrow()[i].widget.get_opacity_();
                    Ok(ok)
                }))?
            )?;
            table.set(
                "set_opacity",
                lua.create_function(clone!([tags], move |_, amount: f64| {
                    tags.borrow()[i].widget.set_opacity_(amount);
                    Ok(())
                }))?
            )?;
            table.set(
                "get_source",
                lua.create_function(clone!([tags], move |_, ()| {
                    tags.borrow()[i].widget.get_source_();
                    Ok(())
                }))?
            )?;
            table.set(
                "set_source",
                lua.create_function(clone!([tags], move |_, src: String| {
                    let ok = tags.borrow()[i].widget.set_source_(src);
                    Ok(ok)
                }))?
            )?;
            table.set(
                "set_visible",
                lua.create_function(clone!([tags], move |_, visible: bool| {
                    let ok = tags.borrow()[i].widget.set_visible_(visible);
                    Ok(ok)
                }))?
            )?;
            table.set(
                "set_inner_html",
                lua.create_function(clone!([tags, scroll, previous_css_provider, searchbar, current_tab], move |_, html: String| {
                    let new_tags = {
                        tags.borrow()[i].widget.set_inner_html(
                            html,
                            scroll.clone(),
                            previous_css_provider.clone(),
                            searchbar.clone(),
                            current_tab.clone()
                        )
                    };

                    match new_tags {
                        Ok(new_tags) => {
                            for tag in new_tags.borrow_mut().drain(..) {
                                tags.borrow_mut().push(tag);
                            }
                            Ok(())
                        },
                        Err(e) => Err(e)
                    }
                }))?
            )?;
            table.set(
                "append_html",
                lua.create_function(clone!([tags, scroll, previous_css_provider, searchbar, current_tab], move |_, html: String| {
                    let new_tags = {
                        tags.borrow()[i].widget.append_html(
                            html,
                            scroll.clone(),
                            previous_css_provider.clone(),
                            searchbar.clone(),
                            current_tab.clone()
                        )
                    };

                    match new_tags {
                        Ok(new_tags) => {
                            for tag in new_tags.borrow_mut().drain(..) {
                                tags.borrow_mut().push(tag);
                            }
                            Ok(())
                        },
                        Err(e) => Err(e)
                    }
                }))?
            )?;

            if multi {
                global_table.push(table)?;
            } else {
                return Ok(table);
            }
        }
    }

    if multi {
        Ok(global_table)
    } else {
        Err(LuaError::RuntimeError("Tag not found".into()))
    }
}

fn print(_lua: &Lua, msg: LuaMultiValue) -> LuaResult<()> {
    let mut output = String::new();
    for value in msg.iter() {
        match value {
            Value::String(s) => output.push_str(s.to_str().unwrap_or("")),
            Value::Integer(i) => output.push_str(&i.to_string()),
            Value::Number(n) => output.push_str(&n.to_string()),
            Value::Boolean(b) => output.push_str(&b.to_string()),
            def => output.push_str(&format!("{def:#?}")),
        }
    }

    lualog!("lua", output);
    println!("{}", output);
    Ok(())
}

// todo: make this async if shit breaks
pub(crate) async fn run(
    luacode: String,
    tags: Rc<RefCell<Vec<Tag>>>,
    scroll: Rc<RefCell<gtk::ScrolledWindow>>,
    previous_css_provider: Option<CssProvider>,
    searchbar: Rc<RefCell<gtk::SearchEntry>>,
    current_tab: Rc<RefCell<Tab>>,
) -> LuaResult<()> {
    let taburl = { current_tab.borrow().url.clone() };
    let lua = Lua::new_with(
        StdLib::ALL_SAFE,
        LuaOptions::new().catch_rust_panics(true),
    )?;
    let globals = lua.globals();

    let window_table = lua.create_table()?;
    let json_table = lua.create_table()?;
    let query_table = lua.create_table()?;

    let parts: Vec<&str> = taburl.splitn(2, '?').collect();

    if parts.len() == 2 {
        let query_params = parts[1];

        let pairs: Vec<&str> = query_params.split('&').collect();
        for pair in pairs {
            let key_value: Vec<&str> = pair.split('=').collect();
            if key_value.len() == 2 {
                let key = key_value[0];
                let value = key_value[1];

                query_table.set(key, value)?;
            }
        }
    }

    let fetchtest = lua.create_async_function(|lua, params: LuaTable| async move {
        // I LOVE MATCH STATEMENTSI LOVE MATCH STATEMENTSI LOVE MATCH STATEMENTSI LOVE MATCH STATEMENTSI LOVE MATCH STATEMENTSI LOVE MATCH STATEMENTS
        let uri = match params.get::<_, String>("url") {
            Ok(url) => url,
            Err(_) => return Err(LuaError::RuntimeError("url is required".into())),
        };
        let method = match params.get::<_, String>("method") {
            Ok(method) => method,
            Err(_) => return Err(LuaError::RuntimeError("method is required".into())),
        };
        let headers = match params.get::<_, LuaTable>("headers") {
            Ok(headers) => headers,
            Err(_) => return Err(LuaError::RuntimeError("headers is required".into())),
        };

        let body_str = match params.get::<_, String>("body") {
            Ok(body) => body,
            Err(_) => "{}".to_string(),
        };

        let mut headermap = HeaderMap::new();

        for header in headers.pairs::<String, String>() {
            let (key, value) = header.unwrap_or(("".to_string(), "".to_string()));

            headermap.insert(
                HeaderName::from_bytes(key.as_ref()).unwrap(),
                HeaderValue::from_str(&value).unwrap(),
            );
        }

        let handle = thread::spawn(move || {
            let client = reqwest::blocking::Client::new();

            let req = match method.as_str() {
                "GET" => client.get(uri).headers(headermap),
                "POST" => client.post(uri).headers(headermap).body(body_str),
                "PUT" => client.put(uri).headers(headermap).body(body_str),
                "DELETE" => client.delete(uri).headers(headermap).body(body_str),
                _ => return format!("Unsupported method: {}", method).into(),
            };

            let res = match req.send() {
                Ok(res) => res,
                Err(e) => {
                    return format!("Failed to send request: {}", e).into();
                }
            };

            let errcode = Rc::new(RefCell::new(res.status().as_u16()));

            let text = res.text().unwrap_or_default();
            let body = serde_json::from_str(&text);

            let result = match body {
                Ok(body) => body,
                Err(e) => {
                    let errcode_clone = Rc::clone(&errcode);

                    lualog!("lua", format!("INFO: failed to parse JSON from response body: {}", e));
                    let mut map: Map<String, serde_json::Value> = Map::new();

                    map.insert("status".to_owned(), serde_json::Value::Number(serde_json::Number::from_f64(*errcode_clone.borrow() as f64).unwrap()));
                    map.insert("content".to_owned(), serde_json::Value::String(text));

                    serde_json::Value::Object(map)
                }
            };

            result
        });

        let json = match handle.join() {
            Ok(json) => json,
            Err(_) => {
                lualog!(
                    "error",
                    format!("Failed to join request thread at fetch request. Originates from the Lua runtime. Returning null.")
                );
                serde_json::Value::Null
            }
        };

        lua.to_value(&json)
    })?;

    let json_stringify =
        lua.create_function(|lua, table: LuaTable| match serde_json::to_string(&table) {
            Ok(value) => Ok(lua.to_value(&value)?),
            Err(_) => {
                lualog!(
                    "error",
                    format!("Failed to stringify JSON. Returning null.")
                );
                Ok(lua.null())
            }
        })?;

    let json_parse = lua.create_function(|lua, json: String| {
        match serde_json::from_str::<serde_json::Value>(&json) {
            Ok(value) => Ok(lua.to_value(&value)?),
            Err(_) => {
                lualog!("error", format!("Failed to parse JSON. Returning null."));
                Ok(lua.null())
            }
        }
    })?;

    window_table.set("link", taburl.clone())?;
    window_table.set("query", query_table)?;

    globals.set("__script_path", taburl.clone())?;
    let require = lua.create_async_function(move |lua, module: String| {
        let taburl = taburl.clone();
        async move {
            let script_path: String = lua.globals().get("__script_path")?;
            if let Ok(mut uri) = url::Url::parse(&script_path) {
                if let Ok(url) = url::Url::parse(&module) {
                    uri = url;
                } else {
                    if let Ok(mut segments) = uri.path_segments_mut() {
                        segments.pop_if_empty();
                        segments.push("");
                    }
                    if let Ok(url) = uri.join(&module) {
                        uri = url;
                    }
                }
                println!("{}", uri.to_string());

                let result = if uri.scheme() == "file" && taburl.starts_with("file") {
                    if let Ok(path) = uri.to_file_path() {
                        if let Ok(contents) = std::fs::read_to_string(path) {
                            contents
                        } else {
                            lualog!("error", format!("file does not exist"));
                            return Ok(lua.null());
                        }
                    } else {
                        lualog!("error", format!("invalid file path"));
                        return Ok(lua.null());
                    }
                } else {{
                    let uri = uri.clone();
                    let handle = thread::spawn(move || {
                        let client = reqwest::blocking::Client::new();
                        let req = client.get(if let Ok(_) = url::Url::parse(&module) {
                            module
                        } else {
                            if uri.domain().unwrap_or("").to_lowercase() == "github.com" {
                                crate::b9::html::get_github_url(uri.to_string())
                            } else { 
                                uri.to_string()
                            }
                        });
                        let res = match req.send() {
                            Ok(res) => res,
                            Err(e) => {
                                return format!("Failed to send request: {}", e).into();
                            }
                        };

                        let text = res.text().unwrap_or_default();
                        text
                    });

                    match handle.join() {
                        Ok(result) => result,
                        Err(_) => {
                            lualog!(
                                "error",
                                format!("Failed to join request thread at fetch request. Originates from the Lua runtime. Returning null.")
                            );
                            "null".to_string()
                        }
                    }
                }};

                if let Err(e) = lua.sandbox(true) {
                    lualog!("error", format!("failed to enable sandbox: {}", e));
                    return Err(LuaError::runtime("failed to enable sandbox"));
                } else {
                    let file = uri.to_string();
                    if let Ok(mut segments) = uri.path_segments_mut() {
                        segments.pop();
                    }
                    lua.globals().set("__script_path", uri.to_string())?;
                    let result = lua.load(result)
                        .set_name(file)
                        .eval::<LuaValue>();
                    lua.globals().set("__script_path", script_path)?;
                    return result;
                }
            }

            lualog!("error", "invalid url");
            Ok(lua.null())
        }
    })?;

    json_table.set("stringify", json_stringify)?;
    json_table.set("parse", json_parse)?;

    globals.set("print", lua.create_function(print)?)?;
    globals.set(
        "get",
        lua.create_function(move |lua, (class, multiple): (String, Option<bool>)| {
            get(lua, class, tags.clone(), 
                scroll.clone(),
                previous_css_provider.clone(),
                searchbar.clone(),
                current_tab.clone(),
                multiple.unwrap_or(false))
        })?,
    )?;
    globals.set(
        "set_timeout",
        lua.create_function(move |lua, (func, ms): (LuaOwnedFunction, u64)| {
            set_timeout(lua, func, ms)
        })?,
    )?;
    globals.set(
        "clear_timeout",
        lua.create_function(move |_lua, id: i32| clear_timeout(id))?,
    )?;
    globals.set("fetch", fetchtest)?;
    globals.set("json", json_table)?;
    globals.set("window", window_table)?;
    globals.set("require", require)?;

    if let Err(e) = lua.sandbox(true) {
        lualog!("error", format!("failed to enable sandbox: {}", e));
        Err(LuaError::runtime("failed to enable sandbox"))
    } else {
        let ok = lua.load(luacode).eval::<LuaMultiValue>();

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
}

// UTILS
fn gtk_buffer_to_text(buffer: &gtk::TextBuffer) -> String {
    let (start, end) = buffer.bounds();
    let text = buffer.text(&start, &end, true);
    text.to_string()
}
// IMPLEMENTATIONS

// h1, h2, h3, h4, h5, h6
impl Luable for gtk::Label {
    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents_(&self) -> String {
        self.text().to_string()
    }
    fn set_visible_(&self, visible: bool) {
        self.set_visible(visible);
    }
    fn get_opacity_(&self) -> f64 {
        self.opacity()
    }
    fn set_opacity_(&self, amount: f64) {
        self.set_opacity(amount);
    }

    fn set_contents_(&self, contents: String) {
        self.set_text(&contents);
        self.style();
    }
    fn _on_click(&self, func: &LuaOwnedFunction) {
        let gesture = gtk::GestureClick::new();

        let a = Rc::new(func.clone());

        gesture.connect_released(move |_, _, _, _| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                lualog!("error", e.to_string());
            }
        });

        self.add_controller(gesture)
    }
}

// select
impl Luable for gtk::DropDown {
    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents_(&self) -> String {
        "".to_string()
    }
    fn set_visible_(&self, visible: bool) {
        self.set_visible(visible);
    }
    fn get_opacity_(&self) -> f64 {
        self.opacity()
    }
    fn set_opacity_(&self, amount: f64) {
        self.set_opacity(amount);
    }
    fn _on_click(&self, func: &LuaOwnedFunction) {
        let gesture = gtk::GestureClick::new();

        let a = Rc::new(func.clone());

        gesture.connect_released(move |_, _, _, _| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                lualog!("error", e.to_string());
            }
        });

        self.add_controller(gesture)
    }
}

// a
impl Luable for gtk::LinkButton {
    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents_(&self) -> String {
        self.label().unwrap_or("".into()).to_string()
    }
    fn get_href_(&self) -> String {
        self.uri().to_string()
    }
    fn get_opacity_(&self) -> f64 {
        self.opacity()
    }
    fn set_opacity_(&self, amount: f64) {
        self.set_opacity(amount);
    }
    fn set_visible_(&self, visible: bool) {
        self.set_visible(visible);
    }
    fn set_contents_(&self, contents: String) {
        self.set_label(&contents);
        self.style();
    }

    fn set_href_(&self, href: String) {
        self.set_uri(&href);
    }

    fn _on_click(&self, func: &LuaOwnedFunction) {
        let gesture = gtk::GestureClick::new();

        let a = Rc::new(func.clone());

        gesture.connect_released(move |_, _, _, _| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                lualog!("error", e.to_string());
            }
        });

        self.add_controller(gesture)
    }
}

// div
impl Luable for gtk::Box {
    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents_(&self) -> String {
        "".to_string()
    }
    fn get_opacity_(&self) -> f64 {
        self.opacity()
    }
    fn set_inner_html(
        &self,
        html: String,
        scroll: Rc<RefCell<gtk::ScrolledWindow>>,
        previous_css_provider: Option<CssProvider>,
        searchbar: Rc<RefCell<gtk::SearchEntry>>,
        current_tab: Rc<RefCell<Tab>>,
    ) -> LuaResult<Rc<RefCell<Vec<Tag>>>> {
        modify_inner_html(html, self, &scroll, &previous_css_provider, &searchbar, &current_tab, false)
    }
    fn append_html(
        &self,
        html: String,
        scroll: Rc<RefCell<gtk::ScrolledWindow>>,
        previous_css_provider: Option<CssProvider>,
        searchbar: Rc<RefCell<gtk::SearchEntry>>,
        current_tab: Rc<RefCell<Tab>>,
    ) -> LuaResult<Rc<RefCell<Vec<Tag>>>> {
        modify_inner_html(html, self, &scroll, &previous_css_provider, &searchbar, &current_tab, true)
    }
    fn set_visible_(&self, visible: bool) {
        self.set_visible(visible);
    }
    fn set_opacity_(&self, amount: f64) {
        self.set_opacity(amount);
    }
    fn _on_click(&self, func: &LuaOwnedFunction) {
        let gesture = gtk::GestureClick::new();

        let a = Rc::new(func.clone());

        gesture.connect_released(move |_, _, _, _| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                lualog!("error", e.to_string());
            }
        });

        self.add_controller(gesture)
    }
}

// textarea
impl Luable for gtk::TextView {
    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents_(&self) -> String {
        let buffer = self.buffer();
        gtk_buffer_to_text(&buffer)
    }
    fn get_opacity_(&self) -> f64 {
        self.opacity()
    }
    fn set_opacity_(&self, amount: f64) {
        self.set_opacity(amount);
    }
    fn set_contents_(&self, contents: String) {
        self.buffer().set_text(&contents);
    }
    fn set_visible_(&self, visible: bool) {
        self.set_visible(visible);
    }
    fn _on_click(&self, func: &LuaOwnedFunction) {
        let gesture = gtk::GestureClick::new();

        let a = Rc::new(func.clone());

        gesture.connect_released(move |_, _, _, _| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                lualog!("error", e.to_string());
            }
        });

        self.add_controller(gesture)
    }
    fn _on_input(&self, func: &LuaOwnedFunction) {
        let a = Rc::new(func.clone());

        self.buffer().connect_changed(move |s| {
            if let Err(e) = a.call::<_, ()>(gtk_buffer_to_text(s)) {
                lualog!("error", e.to_string());
            }
        });
    }
}

// hr
impl Luable for gtk::Separator {
    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents_(&self) -> String {
        "".to_string()
    }
    fn get_opacity_(&self) -> f64 {
        self.opacity()
    }
    fn set_opacity_(&self, amount: f64) {
        self.set_opacity(amount);
    }
    fn set_visible_(&self, visible: bool) {
        self.set_visible(visible);
    }
    fn _on_click(&self, func: &LuaOwnedFunction) {
        let gesture = gtk::GestureClick::new();

        let a = Rc::new(func.clone());

        gesture.connect_released(move |_, _, _, _| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                lualog!("error", e.to_string());
            }
        });

        self.add_controller(gesture)
    }
}

// img
impl Luable for gtk::Picture {
    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents_(&self) -> String {
        "".to_string()
    }
    fn get_opacity_(&self) -> f64 {
        self.opacity()
    }
    fn set_opacity_(&self, amount: f64) {
        self.set_opacity(amount);
    }
    fn set_visible_(&self, visible: bool) {
        self.set_visible(visible);
    }
    fn get_source_(&self) -> String {
        self.alternative_text()
            .unwrap_or(GString::new())
            .to_string()
    }
    fn set_source_(&self, source: String) {
        let stream = match crate::b9::html::fetch_image_to_pixbuf(source.clone()) {
            Ok(s) => s,
            Err(e) => {
                println!("ERROR: Failed to load image: {}", e);
                return;
            }
        };

        self.set_paintable(Some(&gtk::gdk::Texture::for_pixbuf(&stream)));
        self.set_alternative_text(Some(&source))
    }
    fn _on_click(&self, func: &LuaOwnedFunction) {
        let gesture = gtk::GestureClick::new();

        let a = Rc::new(func.clone());

        gesture.connect_released(move |_, _, _, _| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                lualog!("error", e.to_string());
            }
        });
    }
}

// input
impl Luable for gtk::Entry {
    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents_(&self) -> String {
        self.text().to_string()
    }
    fn get_opacity_(&self) -> f64 {
        self.opacity()
    }
    fn set_opacity_(&self, amount: f64) {
        self.set_opacity(amount);
    }
    fn set_visible_(&self, visible: bool) {
        self.set_visible(visible);
    }
    fn set_contents_(&self, contents: String) {
        self.buffer().set_text(contents);
    }
    fn _on_click(&self, func: &LuaOwnedFunction) {
        let gesture = gtk::GestureClick::new();

        let a = Rc::new(func.clone());

        gesture.connect_released(move |_, _, _, _| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                lualog!("error", e.to_string());
            }
        });

        self.add_controller(gesture)
    }
    fn _on_submit(&self, func: &LuaOwnedFunction) {
        let a = Rc::new(func.clone());

        self.connect_activate(move |entry| {
            let content = entry.buffer().text().to_string();

            if let Err(e) = a.call::<_, ()>(content) {
                lualog!("error", e.to_string());
            }
        });
    }
    fn _on_input(&self, func: &LuaOwnedFunction) {
        let a = Rc::new(func.clone());

        self.connect_changed(move |entry| {
            let content = entry.buffer().text().to_string();

            if let Err(e) = a.call::<_, ()>(content) {
                lualog!("error", e.to_string());
            }
        });
    }
}

// button
impl Luable for gtk::Button {
    fn get_css_name(&self) -> String {
        self.css_name().to_string()
    }

    fn get_contents_(&self) -> String {
        self.label().unwrap_or("".into()).to_string()
    }
    fn get_opacity_(&self) -> f64 {
        self.opacity()
    }
    fn set_opacity_(&self, amount: f64) {
        self.set_opacity(amount);
    }
    fn set_contents_(&self, contents: String) {
        self.set_label(&contents);
    }
    fn set_visible_(&self, visible: bool) {
        self.set_visible(visible);
    }

    fn _on_click(&self, func: &LuaOwnedFunction) {
        let a = Rc::new(func.clone());

        self.connect_clicked(move |_| {
            if let Err(e) = a.call::<_, ()>(LuaNil) {
                lualog!("error", e.to_string());
            }
        });
    }
}
