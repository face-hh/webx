# Luau

Luau is a typed scripting language derived from Lua. Instead of JavaScript, we use Luau to power WebX.

{% hint style="danger" %}
### Requires reviewing
This part of the documentation is not finished / requires reviewing. [See the repo.](https://github.com/face-hh/webx/blob/main/docs/).
{% endhint %}

## The fundamentals: SET and GET.

The Luau API is as simple as learning two things: you can SET and you can GET. You can GET to access elements from your page's HTML++, and you can SET to modify them.

{% hint style="info" %}
### You come from JavaScript?
GET would be equivalent to your `document.*` query selectors and getElementById / ClassName, while SET would be equivalent to give a value to props like "href", "opacity", and so on.
{% endhint %}

### GET

In fact, the function to get an element is just called "get".

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}

```lua
local myItem = get("myItem")
```

{% endcode %}

> Much shorter compared to JS :wink:

{% code title="javascript.js" overflow="wrap" lineNumbers="true" %}

```js
// too long
document.querySelector("h1")
// even longer!?
document.getElementByClassName("myItem")
```

{% endcode %}

Note that we use the same function to get items by their tag name or by their class.

**Remember the end of the HTML++ page, where we told you to avoid setting a class name with the same name as a tag?** It's because of this, `get("a")` will get an anchor, so you should not give an item a class name of `class="a"`.

> What about "`querySelectorAll()`?

**Just add "true" to the `get` function to get all the elements of the same class / tag name.

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
myanchor.set_href("buss://skibidi.rizz")
```

{% endcode %}

It's that easy! *Scroll to the bottom for a list of available SET and GET directives.*

### Events

The Luau API has also support for detecting **events**. Similar to JS's `onclick`. To use them, create a function that's called on an event, just like you would do in JS. Example:

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}

```lua
get("a").on_click(function()
    print("The anchor got clicked!!!")
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

Variable names are self explanatory, you give the function the `URL` you want to fetch, the HTTP method you want to use, which can be "GET", "POST", "DELETE", etc..., the headres of the HTTP request, and the body, which would be the content itself of your request.

### Full lists
All the `get_{x}`, `set_{x}` and `on_{x}` available functions.

{% hint style="success" %}
On every function, whenever `x` is expected to be a string or a number, it can always be passed both as a string/number directly or as a Lua variable.
{% endhint %}

#### GET

| <div style="width: 150px;">Function</div> | x | y | Return | Explanation |
| -------- | - | - | ------ | ----------- |
| `get(x, y)` | `x` must be the tag name or class name of the target item. It should be a string. | Can be `true` or can just not be passed at all. In theory it shouldn't be passed as `false`. | No return. | Allows to get items to interact with them. Gets an element using it's tag name or class name. If you pass true as the second argument, every instance with the same class / every instance of the tag will be selected at once. If not, the first coincidence will be chosen. |

#### GETTING FUNCTIONS
| <div style="width: 150px;">Function</div> | Return | Explanation |
| -------- | ------ | ----------- |
| `get_contents()` | If there is any kind of string content (like text) inside of the target item, it returns it as a string. If not, returns an empty string (`""`). If there is both text and HTML++ tags, it will only return the text, and if there are tags but no text, will return the empty string. Won't return text inside of nested tags (nested tag = tag inside of the target item's tag). | Gets the text content of any item. |
| `get_href()` | If `href` exists in the target item, returns it as a string. If not, returns an empty string (`""`). | Gets the `href` value of an anchor. |
| `get_source()` | If `src` exists in the target item, returns it as a string. If not, returns an empty string (`""`). | Gets the `src` value of an image. |
| `get_opacity()` | If `opacity` exists, returns it as a number. Keep in mind if you don't give an opacity value to an item, it defaults to `1`, so it can't be `null` nor an empty string. | Gets the `opacity` value of any item. |

Example usage of GET and GETTING FUNCTIONS

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}

```lua
-- GET
local test = get("myclass")

-- GETTING FUNCTIONS
local opacity = test.get_opacity()
-- can also be called this way
local content = get("h1").get_contents()

-- GET everything
local all_parragraphs = get("p", true)
```

{% endcode %}

#### SET
| <div style="width: 150px">Function</div> | x | Explanation |
| -------- | - | ----------- |
| `set_contents(x)` | `x` must be the name of the target tag or class name. It should be a string. | Gets the text content of any item. |
| `set_href(x)` | `x` must be the URL you want to set the `href` property to. It should be a string. | Sets the `href` value of an anchor. |
| `set_source(x)` | `x` must be the URL you want to set the `src` property to. It should be a string. | Sets the `src` value of an image. |
| `set_opacity(x)` | `x` must be the value you want to set the `opacity` property to. It should be a float between 0 and 1. | Sets the `opacity` value of any item. |
| `set_visible(x)` | `x` must be the value you want to set the `visible` property to. It should be a boolean value, `true` or `false`. | :warning: **Upcoming feature** - ***Not released yet (as of B9 1.2.2)*** - Changes if the item is visible or not. If set to false, the item `dissapears`, and if set to `true`, appears. Defaults to `true`. It's just visual, elements never get removed from the code and are always accessible from there.  |

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

| Function | Trigger | Explanation |
| -------- | ------- | ----------- |
| `on_click(function)` | A mouse click. | Supported by all tags. Each click over the item triggers it once. |
| `on_input(function)` | Editing the content of a field. | Supported by `<input>` and `<textarea>` tags. Each change triggers it once. This means, if I type 2 letters and remove 1 (3 changes), it should be called three times. |
| `on_submit(function)` | Submitting the content of a field. Basically hitting `ENTER` key with the field focused. | Supported by `<input>` and `<textarea>` tags. Each hit triggers it once. |

Example usage of EVENTS

{% code title="script.lua" overflow="wrap" lineNumbers="true" %}

```lua
-- first, we get. this time we'll use a different class name, just in case.
local test = get("mybutton")
-- note: no, buttons are *not* supported as a tag, but any item supports the on_click event, so you can just use an anchor as a button with some styling to make it look like one.

-- now, we do stuff when it gets clicked
test.on_click(function()
    test.set_contents("i was clicked!")
end)

-- a very slightly more complex example:

local input = get("input") -- will get an <input> item
local h1 = get("h1")

input.on_submit(function()
    h1.set_contents("your input was: " + input.get_contents())
)
```

{% endcode %}

#### OTHER FUNCTIONS

That's it! You're ready to write fully functional WebX code! However, we're not done yet. Your beautiful code must be published to the WebX somehow, right? Let's find out about that.