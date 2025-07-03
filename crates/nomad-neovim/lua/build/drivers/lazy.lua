--- @type [string]
local message_queue = {}

--- @type nomad.future.Context
local lazy_ctx = {
  -- Lazy already takes care of scheduling a coroutine.resume() to run in the
  -- next tick of the event loop every time we yield, so we don't need to do
  -- anything here.
  --
  -- See https://lazy.folke.io/developers#building for more infos.
  wake = function() end,

  yield = function()
    -- Yield with the message in front of the queue (if any), which will
    -- cause it to be displayed in Lazy's UI.
    local maybe_msg = table.remove(message_queue, 1)
    coroutine.yield(maybe_msg)
  end,
}

--- @type nomad.neovim.build.Driver
return {
  block_on = function(fut)
    local out = fut:await(lazy_ctx)

    -- The future is done, but display any remaining messages before returning.
    for _, msg in ipairs(message_queue) do
      coroutine.yield(msg)
    end

    return out
  end,

  emit = function(message)
    -- Just push the message to the back of the queue, our yield() will take
    -- care of displaying it in the UI.
    message_queue[#message_queue + 1] = message
  end
}
