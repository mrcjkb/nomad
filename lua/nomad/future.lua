--- An executor takes a future and blocks the current thread until the future
--- completes, returning its value.
---
--- @class (exact) nomad.future.Executor
---
--- @field block_on fun(fut: nomad.future.Future<T>): T


--- TODO: docs.
---
--- @class (exact) nomad.future.Context
--- @field wake fun()
--- @field yield fun()


--- A waker is used by a future to notify the executor that it's ready to be
--- polled again.
---
--- @class (exact) nomad.future.Waker
--- @field wake fun(self: nomad.future.Waker)


--- A future is a unit of lazy, asynchronous computation. It can be polled with
--- a waker, and:
---
--- * if the future hasn't yet completed, it uses the waker to schedule itself
---   to be polled again when it's ready to make some progress;
---
--- * if the future has completed, polling it will return the output of the
---   computation.
--- @class (exact) nomad.future.Future<T>: { poll: fun(self: nomad.future.Future<T>, waker: nomad.future.Waker): T? }

local Future = {}
Future.__index = Future

--- @generic T
--- @param poll fun(wake: fun()): T?
--- @return nomad.future.Future<T>
Future.new = function(poll)
  return setmetatable({
    _has_completed = false,
    _poll = poll,
  }, Future)
end

--- @generic T
--- @param self nomad.future.Future<T>
--- @param waker nomad.future.Waker
--- @return T?
function Future:poll(waker)
  return self._poll(self, waker)
end

--- @generic T
--- @param self nomad.future.Future<T>
--- @param ctx nomad.future.Context
--- @return T
function Future:await(ctx)
  if self._has_completed then
    error("called await() on a Future that has already completed")
  end

  while true do
    local poll = self:poll(ctx.waker)
    if poll then
      self._has_completed = true
      return poll
    end
    ctx.yield()
  end
end

local Waker = {}
Waker.__index = Future

--- @param wake fun()
--- @return nomad.future.Waker
Waker.new = function(wake)
  return setmetatable({ _wake = wake }, Waker)
end

--- A waker that does nothing when a`wake()`n.
---
--- @type nomad.future.Waker
Waker.noop = Waker.new(function() end)

function Waker:wake()
  self._wake(self)
end

---@generic T
---@param fun fun(ctx: nomad.future.Context): T
---@return nomad.future.Future<T>
local async = function(fun)
  error("todo")
end

return {
  async = async,
  Future = Future,
  Waker = Waker,
}
