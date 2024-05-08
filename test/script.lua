local btn = get("ok")

print(btn.get_content())
btn.set_content("Hello, World!")
print(btn.get_content())

btn.on_click(function()
    print("Clicked!")
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

get("futurelink").set_href("https://www.duckduckgo.com/")