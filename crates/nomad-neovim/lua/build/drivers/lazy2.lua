local future = require("nomad.future")
local Context = require("nomad.neovim.build.context")
local Driver = require("nomad.neovim.build.driver")

---@type [string]
local message_queue = {}

local emit = function(message)
  -- Just push it to the back of the message queue, the executor will take care
  -- of displaying it in the UI.
  message_queue[#message_queue + 1] = message
end

-- Lazy already takes care of running the `build()` in a coroutine which is
-- `resumed()`d on every tick of the event loop, so we don't need the waker to
-- do anything, and our executor can just keep yielding until the future
-- completes.
--
-- See https://lazy.folke.io/developers#building for more infos.
local executor = future.Executor.new({
  block_on = function(fut)
    local waker = future.Waker.new_dummy()

    while not fut:poll(waker) do
      -- Yield with the message in the front of the queue (if any), which will
      -- cause it to be displayed in Lazy's UI.
      local maybe_msg = table.remove(message_queue, 1)
      coroutine.yield(maybe_msg)
    end

    -- The builder is done, but display any remaining messages before
    -- returning.
    for _, msg in ipairs(message_queue) do
      coroutine.yield(msg)
    end

    return fut:poll(waker)
  end
})

return Driver.new({
  context = Context.new({ emit = emit }),
  executor = executor
})
