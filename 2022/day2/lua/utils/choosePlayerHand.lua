local constants = require("utils.constants")

local function choosePlayerHand(opponent, outcome)
  -- loose
  if outcome == "X" then
    return constants.looseResponse[opponent]
  end
  -- draw
  if outcome == "Y" then
    return constants.drawResponse[opponent]
  end
  -- win
  if outcome == "Z" then
    return constants.winResponse[opponent]
  end
end

return choosePlayerHand
