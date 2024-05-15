# Bussin Web X

An alternative to the World Wide Web (`http(s)://`), with:
- its own **custom browser** written in Rust with [GTK](https://gtk.org/),
- custom HTML, CSS and ***Lua*** engine (yup, **no javascript! 🎉**),
- custom **DNS** allowing websites such as `buss://ohio.rizz`,
- and **search engine** (wip).

![Preview of buss://register.it, the frontend for registering domains](.github_assets/image.png)

# File structure
- `/napture` - The source code for the **browser** Bussin Napture, used to view buss:// sites.
- `/dns` - The source code for the **DNS** (Domain Name System), used for the API at `https://api.buss.lol`
- [registrar](https://github.com/face-hh/webx-registrar) - The source code for `buss://register.it`, frontend for `https://api.buss.lol` made for Bussin Web X. This can also serve as an example for how buss:// sites are made.

# Download and Install
- For now, you have to download [Rust](https://www.rust-lang.org/tools/install)
Then, you just need to open `install-linux` as an executable(if you can't execute it, first do `sudo chmod +x ./install-linux`, then you should be able to install).
- OR, if you are an Arch user (btw) you can easily download it from the AUR (yay -S napture).
- On Windows, it's not recommented to try to install, since you need many libraries and the install procedure is not yet documented(we are working on it :)).

# Register website
To register a website, navigate to `buss://register.it` **through Bussin Napture**.

You will see this interface.
![Preview of buss://register.it, the frontend for registering domains](.github_assets/image.png)

What you need is the **Publish** section.
- for the domain name, choose whatever you want. (example: `duckduckgo`)
- for the TLD, choose one displayed above the `Result will appear...` label. (example: `rizz`)
- for the IP, you can either use:
   - an IP that serves `/index.html` on port 80
   - a GitHub repository that has `index.html`, **outside any folder**. (example: [registrar](https://github.com/face-hh/webx-registrar))

Don't worry! The IP doesn't have to be valid, and you can save the domain for later!

**WARNING**: After creating the domain, you'll be shown a **secret key**. Please make sure to save it as you will need it to Update/Delete your domain.

# HTML guide
The supported tags are: `head`, `title`, `link`, `meta`, `script`, `h1`-`h6`, `div`, `p`, `ul`, `ol`, `li`, `div`, `button`, `hr`, `img`, `input`, `textarea`, `button`, `select`, `option`. Keep in mind their syntax may be different if you're already familiar with HTML5 (i.e. `link` is used for the tab icon). Please check [registrar](https://github.com/face-hh/webx-registrar) or `/napture/test/index.html` for examples.

# CSS guide
The supported properties are:
- `border-color`
- `border-width`
- `border-style`
- `border-radius`
- `padding`
- `direction` (row | column)
- `align-items`: (fill | start | center | end)
- `gap`
- `color`
- `font-size`
- `font-height`
- `font-family`
- `font-weight` (ultralight | light | normal | bold | ultrabold | heavy)
- `underline` (none | single | double | low | error)
- `underline-color`
- `overline` (none | single)
- `overline-color`
- `strikethrough` (false | true)
- `strikethrough-color`
- `margin-left`
- `margin-right`
- `margin-top`
- `margin-bottom`
- `width` (only on `<input>` & `<textarea>`)
- `height` (only on `<input>` & `<textarea>`)

Properties whose value type wasn't specified are either measured in `px`, or are colors (`#fff`, `red`, etc.)

# Lua guide
For those coming from the traditional web...
```diff
- 1. const test = document.querySelector(".classExample");
- 2. test.textContent = "abc";
- 3. (async () => {})()
- 4. test.href = "https://ok.test"
- 5. console.log(test.href)
- 6. test.addEventListener("click", () => {})
- 7. test.addEventListener("submit", () => {})
+ 1. local test = get("classExample")
+ 2. test.set_content("abc");
+ 3. coroutine.wrap(function())
+ 4. test.set_href("buss://register.it")
+ 5. print(test.get_href())
+ 6. test.on_click(function())
+ 7. test.on_submit(function())
```

I believe you'd get a better understand if you explored the [registrar](https://github.com/face-hh/webx-registrar) repository's `script.lua`.

NOTE: Bussin Napture doesn't support `buss://` redirects yet. They will be added in the official release.

# WIP
The rest of the README is work in progress. Thanks for reading!
