local timeout_btn = get("timeout_btn")
local reset_btn = get("reset_btn")
local text = get("text")

timeout_btn.on_click(function()
  set_timeout(function()
    text.set_content("Different Text")
  end, 500)
end)

reset_btn.on_click(function()
  text.set_content("Text")
end)
