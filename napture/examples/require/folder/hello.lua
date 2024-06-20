print(__script_path)
local printtest = require("anotherfolder/print.lua")

return function()
  printtest("Hello, World!")
end
