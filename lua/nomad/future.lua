--- An executor takes a future and blocks the current thread until the future
--- completes, returning its value.
---
--- @class (exact) nomad.future.Executor
---
--- @field block_on fun(fut: nomad.future.Future<T>): T


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

Future.new = function(poll)
  return setmetatable({ _poll = poll }, Future)
end

--- @generic T
--- @param self nomad.future.Future<T>
--- @param waker nomad.future.Waker
--- @return T?
function Future:poll(waker)
  return self._poll(self, waker)
end

--- @generic T, U
--- @param self nomad.future.Future<T>
--- @param handler fun(T): nomad.future.Future<U>|U
--- @return nomad.future.Future<U>
function Future:and_then(handler)
  error("todo")
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

return {
  Future = Future,
  Waker = Waker,
}
