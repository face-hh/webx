-- local btn = get("startbtn")

-- print(btn.get_content())
-- btn.set_content("Hello, World!")
-- print(btn.get_content())

-- btn.on_click(function()
--     print("clicked!")
-- end)

-- get("input").on_submit(function(content)
--     print(content)
-- end)

-- get("input").on_input(function(content)
--     print(content)
-- end)

-- get("textarea").on_input(function(content)
--     print(content)
-- end)


-- get("futurelink").set_href("https://www.duckduckgo.com/")

coroutine.wrap(function()
    local res = fetch("https://httpbin.org/anything?arg0=val0")

    debug(res)
end)()
