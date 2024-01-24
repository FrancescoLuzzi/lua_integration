M = {}

M.name = function()
    return os.getenv("USER") or "Francesco"
end

return M
