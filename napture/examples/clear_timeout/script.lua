local timeout_btn = get("timeout_btn")
local clear_timeout_btn = get("clear_timeout_btn")
local reset_btn = get("reset_btn")
local text = get("text")
local timeout

timeout_btn.on_click(function()
  if timeout ~= nil then
    return
  end

  timeout = set_timeout(function()
    text.set_content("Different Text")
    timeout = nil
  end, 3000)
end)

clear_timeout_btn.on_click(function()
  if timeout == nil then return end
  clear_timeout(timeout)
  timeout = nil
end)

reset_btn.on_click(function()
  text.set_content("Text")
end)
