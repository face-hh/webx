---
outline: deep
prev:
  text: 'Getting started'
  link: 'dev-start'
next:
  text: 'CSS 3.25'
  link: 'css'
---

# HTML++

HTML ++ looks similar to regular HTML5, but with some differences. Note that B9 is a new engine, not all classic HTML5 features are supported.

## Basic structure.

Same as regular HTML5, the `html` tag, with the `head` for metadata and `body` for your page's content.

```html
<html>
  <head>
    <!--head stuff-->
  </head>
  <body>
    <!--body stuff-->
  </body>
</html>
```

## `<head>`: Metadata in HTML ++

A complete WebX head looks like this:

```html:line-numbers=2
<head>
    <title>My cool web</title>
    <link href="https://buss.log/icon.ico">

    <meta name="theme-color" content="#000000">
    <meta name="description" content="My cool web">

    <link href="styles.css">
    <script src="script.lua" />
</head>
```

Let's explain everything. `<title>` and `<meta name="*">` are tags you know from classic HTML5. `<title>` will give a Title to your page, which will be displayed on both the browser's tab and the search results. `<meta name="description" content="*">` will give Dingle a description to show on the search results, and `theme-color` will give your page that specific theme color (unused as of now).

Now, let's go beyond the standard. You might wonder, why do links have no `rel` attribute? And why is the `script` tag self-closing? (which might even be rendered as an error by your IDE).

## Linking in HTML++

You can link images (as much as you like), stylesheets and scripts (only 1 of each type).
:::info
Audio and video support are being worked on, but as of B9 1.2.2 they are not supported. [See PR](https://github.com/face-hh/webx/pull/150).
:::

The thing is, B9 does not use `rel` to define what to do with each thing. It uses the **order** of your tags, being the first tag of each kind the one that will be used for it's specific purpose. In other words: your first `<link>` which hrefs an image will be your site's icon, your first `<link>` which hrefs a stylesheet will be the source for the page's CSS 3.25, and your first `<link>` which hrefs a script will be the source for the page's Lua script.

```html:line-numbers=2
<head>
    <title>My cool web</title>
    <link href="https://buss.log/icon.ico"> <!--This image will be the page's icon--> // [!code highlight]

    <meta name="theme-color" content="#000000">
    <meta name="description" content="My cool web">

    <link href="styles.css"> <!--This CSS file will be the page's styles--> // [!code highlight]
    <!--This script tag will be the page's script-->
    <script src="script.lua" /> // [!code highlight]
</head>

<body>
  <h1>Hey there!</h1>
  <!--This won't be the icon, and will be rendered as a regular image-->
  <link href="https://buss.log/rick-astley.png">

  <!--This script won't be used-->
  <script src="script2.lua" />
</body>
```
:::info
You cannot embed `<style>` tags or write inline scripts in your HTML. Due to that, the script tag must be self closing, even thought your IDE might mark that as an error.
:::

Well, your head is done. Great! Now let's move on onto the body.

## `<body>`: Page's content in HTML++