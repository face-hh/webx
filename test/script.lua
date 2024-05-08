local btn = get("ok")

print(btn.get_content())
btn.set_content("Hello, World!")
print(btn.get_content())

btn.on_click(function(a, b, c, d)
    print("Clicked!")
end)