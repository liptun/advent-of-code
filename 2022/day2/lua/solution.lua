local file = assert(io.open("input.txt", "r"))
local lines = file:lines()

local handPower = {
  A = 1,
  B = 2,
  C = 3,
  X = 1,
  Y = 2,
  Z = 3,
}

local winResponse = {
  A = "Y",
  B = "Z",
  C = "X"
}

local looseResponse = {
  A = "Z",
  B = "X",
  C = "Y"
}

local drawResponse = {
  A = "X",
  B = "Y",
  C = "Z"
}

local function calcOutcome(opponent, player)
  local outcome = {
    player = 0,
    opponent = 0
  }
  local playerPower = handPower[player]
  local opponentPower = handPower[opponent]
  local playerScore = 0
  local opponentScore = 0


  if (opponentPower == playerPower) then
    playerScore = 3
    opponentScore = 3
  else
    if ((opponentPower == 1 and playerPower == 3) or (opponentPower == 3 and playerPower == 1)) then
      opponentPower = opponentPower * -1
      playerPower = playerPower * -1
    end
    if opponentPower > playerPower then
      opponentScore = 6
    else
      playerScore = 6
    end
  end

  outcome.opponent = handPower[opponent] + opponentScore
  outcome.player = handPower[player] + playerScore

  return outcome
end

local function choosePlayerHand(opponent, outcome)
  -- loose
  if outcome == "X" then
    return looseResponse[opponent]
  end
  -- draw
  if outcome == "Y" then
    return drawResponse[opponent]
  end
  -- win
  if outcome == "Z" then
    return winResponse[opponent]
  end
end

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
print("Player scored " .. firstMatchOutcome.player .. " points and opponent " .. firstMatchOutcome.opponent .. " points")

if (firstMatchOutcome.player > firstMatchOutcome.opponent) then
  print("Player wins the game")
elseif (firstMatchOutcome.player == firstMatchOutcome.opponent) then
  print("It's a draw")
else
  print("Player loses the game")
end

print("--- Part Two ---")
print("Player scored " .. secondMatchOutcome.player .. " points and opponent " .. secondMatchOutcome.opponent .. " points")

if (secondMatchOutcome.player > secondMatchOutcome.opponent) then
  print("Player wins the game")
elseif (secondMatchOutcome.player == secondMatchOutcome.opponent) then
  print("It's a draw")
else
  print("Player loses the game")
end
