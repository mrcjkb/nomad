---@class nomad.Option<T>: { _is_some: boolean, _value: T? }

local Option = {}
Option.__index = Option

Option.__tostring = function(self)
  if self:is_some() then
    return "Option.some(" .. tostring(self:unwrap()) .. ")"
  else
    return "Option.none"
  end
end

---@return nomad.Option<any>
Option.none = function()
  local self = setmetatable({}, Option)
  self._is_some = false
  self._value = nil
  return self
end

---@generic T
---@param value T
---@return nomad.Option<T>
Option.some = function(value)
  local self = setmetatable({}, Option)
  self._is_some = true
  self._value = value
  return self
end

function Option:is_some()
  return self._is_some
end

function Option:is_none()
  return not self:is_some()
end

function Option:map(fun)
  return self:is_none() and self or Option.some(fun(self:unwrap()))
end

function Option:unwrap()
  if self._is_some then
    return self._value
  else
    error("called unwrap() on an none value")
  end
end

return Option
