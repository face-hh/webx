local btn = get("startbtn")

print(btn.get_content())
btn.set_content("Hello, World!")
print(btn.get_content())

local ok = set_timeout(function()
	btn.set_content("ok")
end, 5000)

btn.on_click(function()
    print("clicked!")
end)

get("input").on_submit(function(content)
    print(content)
end)

get("input").on_input(function(content)
    print(content)
end)

get("textarea").on_input(function(content)
    print(content)
end)

-- get("futurelink").set_href("https://www.duckduckgo.com/")

coroutine.wrap(function()
	local res = fetch({
		url = "http://127.0.0.1:3000/",
		method = "POST",
		headers = { ["Content-Type"] = "application/json" },
		body = '{ "test": 3 }',
	})

	print("hlelo", { hello = true })
end)()