# WebX Developer Guide

## Welcome to WebX development!
{% hint style="info" %}
Note this is guide is up to date for **B9 ENGINE - v1.2.2** (Napture and B9's version number are synchronized). If the browser gets a new release, wait for someone to commit / PR an update to the docs ([or make a commit yourself, if you feel like helping](https://github.com/face-hh/webx/tree/master/docs/)).
{% endhint %}

### What we'll see
WebX websites are developed with HTML++, CSS3.25, and [Luau](https://luau-lang.org). In this guide, you'll basically find:
- Full syntax of these languages, explained.
- How to register a domain and upload your website to WebX.
- Additional details on how Bussin's API works.

### File structure
Let's start with the basics. In order to make a web, you'll need to make a project with the following file structure:
```
/
- index.html*
- styles.css
- script.lua
```
{% hint style="warning" %}
*The index file **must** be called "index.html".
{% endhint %}

{% hint style="warning" %}
As of version 1.2.2, WebX does **not** support routing, so no multiple pages (`/index.html`, `/hello.html`) are supported.
{% endhint %}

### Don't get confused
And hey, now that we talk about versioning, you might want to know this:
{% hint style="success" %}
### Don't get confused with naming
Napture and B9 are connected, but they are not the same!
- **WebX** - The entire project, including the DNS and all that stuff.
- **Napture** - The browser itself, used to render websites using the B9 engine.
- **B9 engine** (or just "B9") - Napture's rendering engine, which renders the HTML++, CSS 3.25, and Luau. Most features will need to be supported by *this*, not by the browser itself.
They both have the same version number. **Currently, the Latest version is v1.2.2**.

*If an update releases and the documentation isn't up to date yet, you should see an alert at the beggining of this page. Contributors will try to keep this documentation up to date as soon as updates get released.*
{% endhint %}

### Dev menus and settings
Plus, before starting, you might want to note these keybinds somewhere:
| `CONTROL` + `SHIFT` + `S` | `CONTROL` + `SHIFT` + `P` | `CONTROL` + `SHIFT` + `I` |
| ------------------------- | ------------------------- | ------------------------- |
| This will open Napture's Settings. | This will open *Napture Logs*. Here you'll see logs of what's going on with your own code. | This will open the *GTK Inspector*. Here you'll see logs of what's going on with GTK (the UI toolkit that powers B9). |

Now, let's start! Go to the next pages to start learning how to write your own HTML++, CSS 3.25, and Luau.