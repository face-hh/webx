# Getting started

## Welcome to Web X development

{% hint style="info" %}
Note this is guide is up to date for **B9 ENGINE - v1.3.0** (Napture and B9's version number are synchronized). If the browser gets a new release, wait for someone to commit / PR an update to the docs.
{% endhint %}

### What we'll see

Web X websites are developed with HTML++, CSS 3.25, and [Luau](https://luau-lang.org) (being HTML++ and CSS 3.25 basically HTML and CSS with a few changes). In this guide, you'll basically find:

* Full syntax explained.
* How to register a domain and upload your website to Web X.
* Additional details on how Bussin's API works.

### File structure

Let's start with the basics. In order to make a web, you'll need to make a project with the following file structure:

{% code title="Your project's root" overflow="wrap" lineNumbers="false" %}

```txt
/
- index.html
- styles.css
- script.lua
```

{% endcode %}

{% hint style="warning" %}
The index file **must** be called "index.html".
{% endhint %}

Routing is supported, so if you want to have multiple pages (like `buss://example.fr/secondpage`), you can do that by creating folders with the name you want to give to the page, and creating another `index.html` file inside of that folder.

{% code title="Your project's root" overflow="wrap" lineNumbers="false" %}

```txt
/
- index.html
- styles.css
- script.lua
    /second-page
    - index.html
```

{% endcode %}

Another thing you might want to take into account as we update B9 / Napture, is actually what does each name mean.

{% hint style="success" %}

#### Don't get confused with naming

Napture and B9 are connected, but they are not the same!

* **Web X** - The entire project, including the DNS and all that stuff.
* **Napture** - The browser itself, used to render websites using the B9 engine.
* **B9 engine** (or just "B9") - Napture's rendering engine, which renders the HTML, CSS, and Luau. Most features will need to be supported by _this_, not by the browser itself. They both have the same version number. **Currently, the Latest version is v1.3.0**.

{% endhint %}

### Dev menus and settings

Plus, before starting, you might want to note these keybinds somewhere:

| `CONTROL` + `SHIFT` + `S`          | `CONTROL` + `SHIFT` + `P`                                                                                                                 | `CONTROL` + `SHIFT` + `I`                                                 | `CONTROL` + `SHIFT` + `H`                                   |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- | ------------------------------------------------------------ |
| This will open Napture's Settings. | This will open _Napture Logs_. This is made for developers, but it may help you understand why a page isn't working in case that happens. | This will open the built-in _GTK Inspector_. This is made for developers. | This will show the browser's history. |

And another thing: you probably develop from Visual Studio Code, as it's the most used code editor. For that, we recomend [KashTheKing](https://github.com/KashTheKing)'s [WebX Lua types extension](https://github.com/KashTheKing/webxluatypes), which will help you get better autocompletion and diagnostics when you write code for the WebX.

Now, let's start! Go to the next pages to start learning how to write your own HTML++, CSS 3.25, and Lua.
