# Lua

Luau is a typed scripting language derived from Lua. Instead of JavaScript, we use Luau to power Web X.

{% hint style="info" %}
Note: We say "Luau" as it is what we use in our backend, but you do not need to install it. **You can write regular Lua and your code will work fine.**
{% endhint %}

## SET and GET.

The Luau API is as simple as learning two things: you can SET and you can GET. You can GET to access elements from your page's HTML++, and you can SET to modify them.

{% hint style="info" %}
#### You come from JavaScript?

GET would be equivalent to your `document.*` query selectors and getElementById / getElementbyClassName, while SET would be equivalent to give a value to props like "href", "opacity", and so on.
{% endhint %}

### GET

In fact, the function to get an element is just called "get".

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}
```lua
local my_item = get("my_item")
```
{% endcode %}

> Much shorter compared to JS :wink:

{% code title="javascript.js" overflow="wrap" lineNumbers="true" %}
```js
// too long
document.querySelector("h1")
// even longer!?
document.getElementByClassName("my_item")
```
{% endcode %}

Note that we use the same function to get items by their tag name or by their class.

**Remember the end of the HTML++ page, where we told you to avoid setting a class name with the same name as a tag?** It's because of this, `get("a")` will get an anchor, so you should not give an item a class name of `class="a"`.

> What about "`querySelectorAll()`?

**Just add "true" to the `get` function to get all the elements of the same class / tag name.**

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}
```lua
-- Will give a single anchor
local all_hyperlinks = get("a");
-- Will give all anchors (or hyperlinks, them the way you prefer)
local all_hyperlinks = get("a", true);
```
{% endcode %}

{% hint style="success" %}
When you don't pass `true`, if you select a tag or a class name that is repeated, the first one in the HTML++ will be taken.
{% endhint %}

### SET

Now, to interact with the element, you can use `get_{prop}` **and `set_{prop}`** functions. Here's an example:

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}
```lua
local myanchor = get("a")

-- This will GET the href property
local myurl = myanchor.get_href()
print(myurl)

-- This will SET the href property to a new value
myanchor.set_href("buss://dingle.it")
```
{% endcode %}

It's that easy! _Scroll to the bottom for a list of available SET and GET directives._

### Events

The Luau API has also support for detecting **events**. Similar to JS's `onclick`. To use them, create a function that's called on an event, just like you would do in JS. Example:

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}
```lua
get("button").on_click(function()
    print("The button got clicked!!!")
end)

get("input").on_input(function(content)
    print(content)
end)
```
{% endcode %}

{% hint style="success" %}
`on_input` is only supported by `<input>` and `<textarea>`
{% endhint %}

{% hint style="success" %}
Output from the `print()` function will be logged by Napture Logs. As we told you before, you can open them with `CONTROL` + `SHIFT` + `P`
{% endhint %}

### HTTP Fetching

If you need to make an HTTP request to interact with an API, you can use the `fetch` function. It should look like this:

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}
```lua
local test = true;

local res = fetch({
    url = "https://api.buss.lol/",
    method = "GET",
    headers = { ["Content-Type"] = "application/json" },
    body = '{ "test": ' .. test .. '}'
})
```
{% endcode %}

Variable names are self explanatory, you give the function the `URL` you want to fetch, the HTTP method you want to use, which can be "GET", "POST", "DELETE", etc..., the headers of the HTTP request, and the body, which would be the content itself of your request.

### Full lists

All the `get_{x}`, `set_{x}` and `on_{x}` available functions.

{% hint style="success" %}
On every function, whenever `x` is expected to be a string or a number, it can always be passed both as a string/number directly or as a Lua variable.
{% endhint %}

#### GET

<table data-header-hidden><thead><tr><th></th><th></th><th width="144"></th><th></th><th></th></tr></thead><tbody><tr><td>Function</td><td>x</td><td>y</td><td>Return</td><td>Explanation</td></tr><tr><td><code>get(x, y)</code></td><td><code>x</code> must be the tag name or class name of the target item. It should be a string.</td><td>Can be <code>true</code> or can just not be passed at all.</td><td>A "Tag" class (or more, if <code>true</code>), functions below.</td><td>Allows to get HTML tags to interact with. Gets an element using it's tag name or class name. If you pass true as the second argument, every instance with the same class / every instance of the tag will be selected at once. If not, the first match will be chosen.</td></tr></tbody></table>

#### GETTING FUNCTIONS

| Function         | Return                                                                                                                                                                  | Explanation                           |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| `get_contents()` | If there is any kind of string contents (like text) inside of the target item, it returns it as a string. If not, returns an empty string (`""`).                       | Gets the text content of any item.    |
| `get_href()`     | If `href` exists in the target item, returns it as a string. If not, returns an empty string (`""`).                                                                    | Gets the `href` value of an anchor.   |
| `get_source()`   | If `src` exists in the target item, returns it as a string. If not, returns an empty string (`""`).                                                                     | Gets the `src` value of an image.     |
| `get_opacity()`  | If `opacity` exists, returns it as a number. Keep in mind if you don't give an opacity value to an item, it defaults to `1`, so it can't be `null` nor an empty string. | Gets the `opacity` value of any item. |

Example usage of GET and GETTING FUNCTIONS

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}
```lua
-- GET
local test = get("myclass")

-- GETTING FUNCTIONS
local opacity = test.get_opacity()
-- can also be called this way
local content = get("h1").get_contents()

-- GET every p tag
local all_paragraphs = get("p", true)
```
{% endcode %}

#### SET

| Function          | x                                                                                                                 | Explanation                                                                                                                                                                                                        |
| ----------------- | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `set_contents(x)` | `x` should be a string.                                                                                           | Gets the text content of any item.                                                                                                                                                                                 |
| `set_href(x)`     | `x` must be the URL you want to set the `href` property to. It should be a string.                                | Sets the `href` value of an anchor.                                                                                                                                                                                |
| `set_source(x)`   | `x` must be the URL or base64 that you want to set the `src` property to. It should be a string.                  | Sets the `src` value of an image.                                                                                                                                                                                  |
| `set_opacity(x)`  | `x` must be the value you want to set the `opacity` property to. It should be a float between 0 and 1.            | Sets the `opacity` value of any item.                                                                                                                                                                              |
| `set_visible(x)`  | `x` must be the value you want to set the `visible` property to. It should be a boolean value, `true` or `false`. | :warning: **Upcoming feature** - _**Not released yet (as of B9 1.2.2)**_ - Changes if the item is visible or not. It's just visual, elements never get removed from the code and are always accessible from there. |

Example usage of SET

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}
```lua
-- first, we get
local test = get("myclass")

-- now, we set
test.set_opacity(0.75)
test.set_contents("This text will be set as the content of the element")

-- example: get an anchor and set it's href to the Dingle search engine and its text content to "Search with Dingle"
get("a").set_href("buss://dingle.it")
get("a").set_contents("Search with Dingle")
```
{% endcode %}

#### EVENTS

> Event functions do not have a return. As showed before, put them inside of a function. Every time they get triggered the code of the functions will be executed.

| Function              | Trigger                                                                                  | Explanation                                                                                                                                                                                        |
| --------------------- | ---------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `on_click(function)`  | A mouse click.                                                                           | Supported by all tags. Each click over the item triggers it once. NO ARGUMENTS GIVEN.                                                                                                              |
| `on_input(function)`  | Editing the content of a field.                                                          | Supported by `<input>` and `<textarea>` tags. Each change triggers it once. This means, if I type 2 letters and remove 1 (3 changes), it should be called three times. STRING ARGUMENT (#1) GIVEN. |
| `on_submit(function)` | Submitting the content of a field. Basically hitting `ENTER` key with the field focused. | Supported by `<input>` and `<textarea>` tags. Each hit triggers it once. NO ARGUMENTS GIVEN.                                                                                                       |

Example usage of EVENTS

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}
```lua
local test = get("mybutton")

-- now, we do stuff when it gets clicked
test.on_click(function()
    test.set_contents("i was clicked!")
end)

-- a slightly more complex example:

local input = get("input") -- will get an <input> item
local h1 = get("h1")

input.on_submit(function()
    h1.set_contents("your input was: " + input.get_contents())
)
```
{% endcode %}

#### OTHER FUNCTIONS

| Function   | x                                                           | Return                                                | Explanation                                 |
| ---------- | ----------------------------------------------------------- | ----------------------------------------------------- | ------------------------------------------- |
| `print(x)` | `x` can be any type.                                        | No return.                                            | Will print `x` to Napture Logs.             |
| `fetch(x)` | `x` must be an array with the contents of the HTTP request. | Returns the response of the HTTP request as a string. | Allows to make HTTP requests to fetch APIs. |

{% hint style="info" %}
#### About fetch

This is what `x` (the content of your request) should look like:

{% code title="fetch.lua" overflow="wrap" lineNumbers="true" %}
```lua
local response = fetch({
    url = "https://api.buss.lol/",
    method = "GET", -- HTTP REQUEST METHOD
    headers = { ["Content-Type"] = "application/json" },
    body = '{ "test": ' .. test .. '}' -- REQUEST BODY
})
```
{% endcode %}

Basically `url`, `method`, `headers`, and `body`. Remember that `fetch` will return whatever the HTTP request itself returns (the HTTP response, basically).
{% endhint %}

And that's it! You're ready to write fully functional WebX code! However, we're not done yet. Your beautiful code must be published to the WebX somehow, right? Let's find out about that.
