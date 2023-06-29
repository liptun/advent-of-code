local calcOutcome = require("utils.calcOutcome")
local choosePlayerHand = require("utils.choosePlayerHand")
local printMatchOutcome= require("utils.printMatchOutcome")

local file = assert(io.open("input.txt", "r"))
local lines = file:lines()


local firstMatchOutcome = {
  player = 0,
  opponent = 0
}
local secondMatchOutcome = {
  player = 0,
  opponent = 0
}

for line in lines do
  local t = {}
  for str in string.gmatch(line, "%S") do
    table.insert(t, str)
  end

  local firstOutcome = calcOutcome(t[1], t[2])
  firstMatchOutcome.player = firstMatchOutcome.player + firstOutcome.player
  firstMatchOutcome.opponent = firstMatchOutcome.opponent + firstOutcome.opponent

  local secondOutcome = calcOutcome(t[1], choosePlayerHand(t[1], t[2]))
  secondMatchOutcome.player = secondMatchOutcome.player + secondOutcome.player
  secondMatchOutcome.opponent = secondMatchOutcome.opponent + secondOutcome.opponent
end

file:close()

print("--- Part One ---")
printMatchOutcome(firstMatchOutcome)

print("--- Part Two ---")
printMatchOutcome(secondMatchOutcome)
