---
outline: deep
prev:
  text: 'HTML ++'
  link: 'htmlpp'
next:
  text: 'Luau'
  link: 'luau'
---

# CSS 3.25

CSS 3.25 looks similar to regular CSS 3, but with some differences. Note that B9 is a new engine, not all classic CSS 3 features are supported.

## Basic structure

This is an example of a CSS 3.25 file.

```css
div {
    border-color: #616161;
    border-width: 1px;
    border-style: solid;
    border-radius: 12px;
    padding: 10px;
    direction: row;
    align-items: center;
    gap: 10;
}

h1 {
    padding: 20px;
    color: #444;
}

p {
    color:antiquewhite;
    font-size: 16px;
    line-height: 1.5;
    font-family: Iosevka;
    font-weight: bold;

    border-style: solid;
    border-width: 5px;
    border-color: red;

    border-radius: 12px;
}

h4 {
    font-size: 30px;
}

a {
    color: #007bff;
    font-size: 16px;
    text-decoration: none;
    font-weight: ultralight;
    underline: double;
    underline-color: #ff0000;
    overline: single;
    overline-color: #ff0000;
    strikethrough: true;
    strikethrough-color: #ff0000;
    margin-right: 50px;
}

ul, ol {
    margin-left: 20px;
}

ok {
    color: red;
}

select {
    padding: 5px;
    border-width: 1px;
    border-style: solid;
    border-color: #ccc;
    border-radius: 3px;
    margin-left: 40px;
}

hr {
    border-color: #ccc;
    border-width: 1px;
    border-style: solid;
}
```

Looks like a lot, huh? It's not that complicated. It's just about taking a few things into account and then just checking a list of properties.

## Take into account

1. **No selectors are required:** You don't need to use a dot to select a class (e.g. `.myClass {}`). Since there's no ID's to differentiate with, just put the class name without anything (e.g. `myClass{}`).
2. **Use specific units:** Only **px** for measures and **HEX** (#123456) for colors are supported. No RGBa, no HSL, no em, no viewport units...
3. **Don't touch the body:** As of B9 v1.2.2, styling the body itself might not work properly.

Noted that? Now let's get to the styling features.

## Styling
### Text

| <div style="width:200px">Property</div> | Possible value | Explanation |
| -------- | ----- | ----------- |
| `font-size` | Any `px` value | Gives the text a custom size. There is a default for each text tag (`h1`, `h2`, `p`...) |
| `font-height` | Any `px` value | Gives the text a custom line-height. There is a default for each text tag. |
| `font-family` | String value | Gives the text a custom font. `<link>` does not support font files, so the end user must have that font installed. As of B9 v1.2.2, fallback fonts don't seem to be supported. |
| `font-weight` | Any of the following: *ultralight*, *light*, *normal*, *bold*, *ultrabold*, *heavy* | Gives the text a custom font weight. |
| `color` | Any `HEX` value | Gives the text a custom color. |
### Text underline / overline / strikethrough
> Non-standard, this feature doesn't exist on CSS 3.

| <div style="width:200px">Property</div> | Possible value | Explanation |
| -------- | ----- | ----------- |
| `strikethrough` | `true` or `false` | Gives the text a strikethrough. |
| `font-height` | Any `px` value | Gives the text a custom line-height. There is a default for each text tag. |
| `font-family` | String value | Gives the text a custom font. `<link>` does not support font files, so the end user must have that font installed. As of B9 v1.2.2, fallback fonts don't seem to be supported. |
| `font-weight` | Any of the following: *ultralight*, *light*, *normal*, *bold*, *ultrabold*, *heavy* | Gives the text a custom font weight. |
| `color` | Any `HEX` value | Gives the text a custom color. |
| `color` | Any `HEX` value | Gives the text a custom color. |