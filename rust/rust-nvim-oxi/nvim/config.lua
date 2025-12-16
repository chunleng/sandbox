local script_path = debug.getinfo(1, "S").source:sub(2)
local script_dir = vim.fn.fnamemodify(script_path, ":h")
vim.cmd('set runtimepath+=' .. script_dir)

local calc = require("sample")
print(calc.add(1,2))
