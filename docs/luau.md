---
outline: deep
prev:
  text: 'CSS 3.25'
  link: 'css'
next:
  text: 'Site publishing & Domain registering'
  link: 'dev-publish'
---

# Luau

Luau is a typed scripting language derived from Lua. Instead of JavaScript, we use Luau to power WebX.

:::danger TO-DO
This part of the documentation is not finished / requires reviewing. [See the repo.](https://github.com/face-hh/webx/blob/main/docs/).
:::

## The fundamentals: SET and GET.

The Luau API is as simple as learning two things: you can SET and you can GET. You can GET to access elements from your page's HTML++, and you can SET to modify them.

:::tip You come from JavaScript?
GET would be equivalent to your `document.*` query selectors and getElementById / ClassName, while SET would be equivalent to give a value to props like "href", "opacity", and so on.
:::

### GET

In fact, the function to get an element is just called "get".
```lua
local myItem = get("myItem")
```
> Much shorter compared to JS :wink:

```diff
- document.querySelector("myItem")
- document.getElementByClassName("myItem")
+ local myItem = get("myItem")
```

Note that we use the same function to get items by their tag name or by their class.

**Remember the end of the HTML++ page, where we told you to avoid setting a class name with the same name as a tag?** It's because of this, `get("a")` will get an anchor, so you should not give an item a class name of `class="a"`.

> What about "`querySelectorAll()`?

**Just add "true" to the `get` function to get all the elements of the same class / tag name.
```lua{4}
-- Will give a single anchor
local all_hyperlinks = get("a");
-- Will give all anchors (or hyperlinks, them the way you prefer)
local all_hyperlinks = get("a", true);
```

:::tip
When you don't pass `true`, if you select a tag or a class name that is repeated, the first one in the HTML++ will be taken.
:::

### SET

Now, to interact with the element, you can use `get_{prop}` **and `set_{prop}`** functions. Here's an example:

```lua{4,8}
local myanchor = get("a")

-- This will GET the href property
local myurl = myanchor.get_href()
print(myurl)

-- This will SET the href property to a new value
myanchor.set_href("buss://skibidi.rizz")
```

It's that easy! *Scroll to the bottom for a list of available SET and GET directives.*

### Events

The Luau API has also support for detecting **events**. Similar to JS's `onclick`. To use them, create a function that's called on an event, just like you would do in JS. Example:

```lua
get("a").on_click(function()
    print("The anchor got clicked!!!")
end)

get("input").on_input(function(content)
    print(content)
end)
```

:::info
`on_input` is only supported by `<input>` and `<textarea>`
:::

:::tip
Output from the `print()` function will be logged by Napture Logs. As we told you before, you can open them with `CONTROL` + `SHIFT` + `P`
:::

### HTTP Fetching

If you need to make an HTTP request to interact with an API, you can use the `fetch` function. It should look like this:

```lua
local test = true;

local res = fetch({
    url = "https://api.buss.lol/",
    method = "GET",
    headers = { ["Content-Type"] = "application/json" },
    body = '{ "test": ' .. test .. '}'
})
```
Variable names are self explanatory, you give the function the `URL` you want to fetch, the HTTP method you want to use, which can be "GET", "POST", "DELETE", etc..., the headres of the HTTP request, and the body, which would be the content itself of your request.

### Full lists
All the `get_{x}`, `set_{x}` and `on_{x}` available functions.

:::tip
On every function, whenever `x` is expected to be a string or a number, it can always be passed both as a string/number directly or as a Lua variable.
:::

#### GET
| Function | x | Return | Explanation |
| -------- | - | ------ | ----------- |
| `get_href(x)` | `x` must be the name of the target tag or class name. It should be a string. | If `href` exists in the target item, returns it as a string. If not, returns an empty string (`""`). | Gets the `href` value of an anchor. |
| `get_source(x)` | `x` must be the name of the target tag or class name. It should be a string. | If `src` exists in the target item, returns it as a string. If not, returns an empty string (`""`). | Gets the `src` value of an image. |
| `get_opacity(x)` | `x` must be the name of the target tag or class name. It should be a string. | If `opacity` exists, returns it as a number. Keep in mind if you don't give an opacity value to an item, it defaults to `1`, so it can't be `null` nor an empty string. | Gets the `opacity` value of any item. |

#### SET
| Function | x | Return | Explanation |
| -------- | - | ------ | ----------- |
| `set_href(x)` | `x` must be the URL you want to set the `href` property to. It should be a string. | No return. | Sets the `href` value of an anchor. |
| `set_source(x)` | `x` must be the URL you want to set the `src` property to. It should be a string. | No return. | Sets the `src` value of an image. |
| `set_opacity(x)` | `x` must be the value you want to set the `opacity` property to. It's should be a number between 0 and 1. Decimals supported. | No return. | Sets the `opacity` value of any item. |
| `set_visible(x)` | `x` must be **TO DO** | No return. | **TO DO** |

:::danger TO DO
This part requires reviewing / finishing. [See the repo.](https://github.com/face-hh/webx/blob/main/docs/).
:::

#### EVENTS
> Event functions do not have a return. As showed before, put them inside of a function. Every time they get triggered the code of the functions will be executed.

| Function | x | Trigger | Explanation |
| -------- | - | ------- | ----------- |
| `on_click(x)` | `x` must be the name of the target tag or class name. It should be a string. | A mouse click. | Supported by all tags. Each click over the item triggers it once. |
| `on_input(x)` | `x` must be the name of the target tag or class name. It should be a string. | Editing the content of a field. | Supported by `<input>` and `<textarea>` tags. Each change triggers it once. This means, if I type 2 letters and remove 1 (3 changes), it should be called three times. |
| `on_submit(x)` | `x` must be the name of the target tag or class name. It should be a string. | Submitting the content of a field. Basically hitting `ENTER` key with the field focused. | Supported by `<input>` and `<textarea>` tags. Each hit triggers it once. |

That's it! You're ready to write fully functional WebX code! However, we're not done yet. Your beautiful code must be published to the WebX somehow, right? Let's find out about that.