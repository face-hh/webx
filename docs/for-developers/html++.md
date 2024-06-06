# HTML++

HTML++ looks similar to regular HTML5, but with some differences. Note that B9 is a new engine, not all classic HTML5 features are supported.

## Basic structure.

Same as regular HTML5, the `html` tag, with the `head` for metadata and `body` for your page's content.

{% code title="index.html" overflow="wrap" lineNumbers="true" %}
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
{% endcode %}

## `<head>`: Metadata in HTML++

A complete WebX head looks like this:

{% code title="index.html" overflow="wrap" lineNumbers="true" %}
```html
<head>
    <title>My cool web</title>
    <link href="https://buss.log/icon.ico">

    <meta name="theme-color" content="#000000">
    <meta name="description" content="My cool web">

    <link href="styles.css">
    <script src="script.lua" />
</head>
```
{% endcode %}

Let's explain everything. `<title>` and `<meta name="*">` are tags you know from classic HTML5. `<title>` will give a Title to your page, which will be displayed on both the browser's tab and the search results. `<meta name="description" content="*">` will give Dingle a description to show on the search results, and other meta tags may be used by websites that wish to embed Web X sites.

Now, let's go beyond the standard. You might wonder, why do links have no `rel` attribute? And why is the `script` tag self-closing? (which might even be rendered as an error by your IDE).

## Linking in HTML++

You can link images (as much as you like), stylesheets and scripts (only 1 of each type).

{% hint style="info" %}
Audio and video support are being worked on, but as of B9 1.2.2 they are not supported. [See PR](https://github.com/face-hh/webx/pull/150).
{% endhint %}

The thing is, B9 does not use `rel` to define what to do with each thing. It uses the **order** of your tags, being the first tag of each kind the one that will be used for it's specific purpose. In other words: your first `<link>` has to href an image and will be your site's icon, your second `<link>` has to href a stylesheet and will be the source for the page's CSS, and your `<script>` tag should be the third one, and will be the source for the page's Luau script.

{% hint style="warning" %}
This isn't the standard web, **HTTPS is supported only by the \<script> tag**. Both (CSS & Lua) files must be available locally, in the same path as the `index.html`. As we told you before, the three files must be in the root. HTTP(S) is supported for the icon's link and for images. We recommend using Imgur.
{% endhint %}

{% code title="index.html" overflow="wrap" lineNumbers="true" %}
```html
<head>
    <title>My cool web</title>
    <link href="https://buss.log/icon.ico"> <!--This image will be the page's icon-->

    <meta name="theme-color" content="#000000">
    <meta name="description" content="My cool web">

    <link href="styles.css"> <!--This CSS file will be the page's styles-->
    <!--This script tag will be the page's script-->
    <script src="script.lua" />
</head>

<body>
  <h1>Hey there!</h1>
  <!--This won't be the icon, and will be rendered as a regular image-->
  <link href="https://buss.log/rick-astley.png">

  <!--This script won't be used-->
  <script src="script2.lua" />
</body>
```
{% endcode %}

{% hint style="info" %}
You cannot embed `<style>` tags or write inline scripts in your HTML. Due to that, the script tag must be self closing, even though your IDE might mark that as an error.
{% endhint %}

{% hint style="success" %}
If you mess up something in your head, the title bar will notify you about that. Given image is an example of how it would look like: ![Screenshot](../png3.png)
{% endhint %}

Well, your head is done. Great! Now let's move on onto the body.

## `<body>`: Page's content in HTML++

Currently, you can use the following HTML tags:

* Headers (`<h1>`, `<h2>`, `<h3>`, `<h4>`, `<h5>`, `<h6>`) and paragraphs (`<p>`). Headers will be bigger or smaller based on the number, being 1 the biggest and 6 the smallest.
* Anchors (or hyperlinks) (`<a>`), which can have a `href` property (`<a href="#>`).
  * If the href starts with `buss://`, Napture will open it and redirect the user away from your page. If it starts with anything else, GTK will handle it and automaticaly open your default WWW browser.
* Divisions (`<div>`), basically containers where you can put your tags to organise your page.
* Lists, which can be ordered (`<ol>`) or unordered (`<ul>`). Both can have list items (`<li>`) in them.
* Horizontal rule (`<hr>`), which creates a horizontal line on the entire page.
* Images (`<img>`), with a `src` property for the local path / HTTP URI of the image. They are unresizeable, so your image must already be of the desired size (use any image editing tool for that, e.g. [GIMP](https://www.gimp.org/)).

{% hint style="warning" %}
Additionally, **as of B9 v1.2.2** images are currently broken on _Windows_; and will only work if you follow the [Guide of Compilation for Napture - Windows.](https://github.com/face-hh/webx?tab=readme-ov-file#windows-1)
{% endhint %}

* Inputs (`<input>`) are one-line text fields. You can interact with them with Web X's Lua API.
* Textareas (`<textarea>`) are equivalent to inputs, but they are _multi-line_ text fields instead of _one-line_.
* Dropdowns (`<select>`), which can have options in them (`<option>`).

{% hint style="warning" %}
As of B9 1.2.2, dropdowns are purely decorative at the moment as they don't have a Lua API.
{% endhint %}

We're done with the HTML++, **but you must note one more thing**.

> Every tag that is made for the body (`<h1>`, `<p>`, `<input>`, etc...) has support for a property called "`class`". You might know them for CSS 3 styling, but here they are more important as they are used **for scripting aswell**. Therefore, HTML 5's standard `id` is not supported. Keep that in mind.

{% hint style="info" %}
Never give a class the same name than a supported HTML++ tag. [This is because of how Lua gets elements.](luau.md#get)
{% endhint %}

Now, let's move onto styling.
