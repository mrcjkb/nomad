---@alias nomad.Result<T, E> nomad.result.Result<T, E>

---@class nomad.result.ResultModule
---@field ok fun(value: T): nomad.result.Result<T, any>
---@field err fun(err: E): nomad.result.Result<any, E>

---@class nomad.result.Result<T, E>: { value: T?, error: E? }

local result = {}
result.__index = result

---@return nomad.result.Result
result.new = function(value, error)
  local self = {
    value = value,
    error = error,
  }
  return setmetatable(self, result)
end

function result:is_ok()
  return self.value ~= nil
end

function result:is_err()
  return self.error ~= nil
end

function result:unwrap()
  if self:is_ok() then
    return self.value
  else
    error("called `unwrap()` on an error value")
  end
end

function result:unwrap_err()
  if self:is_err() then
    return self.error
  else
    error("called `unwrap_err()` on an ok value")
  end
end

---@type nomad.result.ResultModule
return {
  ok = function(value) return result.new(value, nil) end,
  err = function(err) return result.new(nil, err) end,
}
