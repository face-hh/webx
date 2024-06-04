---
outline: deep
next:
  text: 'HTML++'
  link: 'htmlpp'
---

# WebX Developer Guide

:::info
Note this is guide is up to date for **B9 ENGINE - v1.2.2** (Napture and B9's version number are syncronised). If the browser gets a new release, wait for someone to commit / PR an update to the docs ([or make a commit yourself, if you feel like helping](https://github.com/face-hh/webx/blob/main/docs/)).
:::

WebX websites are developed with HTML++, CSS3.25, and [Lua 5.4](https://lua.org). In this guide, you'll basically find:
- Full syntax of these languages, explained.
- How to register a domain and upload your website to WebX.
- Additional details on how Bussin's API works.

Let's start with the basics. In order to make a web, you'll need to make a project with the following file structure:
```
/
- index.html*
- styles.css
- script.lua
```
:::warning
*The index file **must** be called "index.html".
:::

:::warning
As of version 1.2.2, WebX does **not** support routing, so no multiple pages (`/index.html`, `/hello.html`) are supported.
:::

Go to the next page to start writing your HTML++, CSS 3.25, and Lua.