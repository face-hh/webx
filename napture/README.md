# Bussin Napture

This folder contains the source code for **Bussin Napture**, the official Bussin Web X **browser**.

The file structure:

- `src/b9` - B9 is the codename for Napture's rendering engine. It is responsible for handling, parsing, and rendering HTML++, CSS 3.25 and Lua.

Other info:

- To support more tags, take a look at [GTK Widget Galery](https://docs.gtk.org/gtk4/visual_index.html). You'd have to modify the `render_body` function in `src/b9/html.rs`.
- To support more CSS properties, take a look at `src/b9/css.rs`. Each element has different methods & properties, so you'll have to add a default value.
- To add more Lua functions, take a look at `src/b9/lua.rs`.
