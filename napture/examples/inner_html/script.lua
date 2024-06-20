local btn = get("btn")
local btn2 = get("btn2")
local reset_btn = get("reset_btn")
local text = get("text")

btn.on_click(function()
  text.append_html("<div class=\"test\"><a href=\"buss://dingle.it\">dingle</a></div>")
end)

btn2.on_click(function()
  text.set_inner_html("<div class=\"test\"><a href=\"buss://dingle.it\">test</a></div>")
  print(get(test))
end)

reset_btn.on_click(function()
  text.set_inner_html("<p>Text</p>")
end)
