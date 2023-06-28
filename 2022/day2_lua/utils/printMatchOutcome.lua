-- ANSI colors
local color = {
  red = "\27[1;31m",
  yellow = "\27[1;33m",
  green = "\27[1;32m",
  reset = "\27[0m"
}

local function printMatchOutcome(matchOutcome)
  print("Player scored " ..
    color.yellow ..
    matchOutcome.player ..
    color.reset .. " points and opponent " .. color.yellow .. matchOutcome.opponent .. color.reset .. " points")
  if (matchOutcome.player > matchOutcome.opponent) then
    print("Player " .. color.green .. "wins" .. color.reset .. " the game")
  elseif (matchOutcome.player == matchOutcome.opponent) then
    print("It's a " .. color.yellow .. "draw" .. color.reset)
  else
    print("Player " .. color.red .. "loses" .. color.reset .. " the game")
  end
end

return printMatchOutcome
