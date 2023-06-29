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

local matchOutcome = {
  player = 0,
  opponent = 0
}

for line in lines do
  local t = {}
  for str in string.gmatch(line, "%S") do
    table.insert(t, str)
  end
  local outcome = calcOutcome(t[1], t[2])
  matchOutcome.player = matchOutcome.player + outcome.player
  matchOutcome.opponent = matchOutcome.opponent + outcome.opponent
end

file:close()

print("Player scored " .. matchOutcome.player .. " points and opponent " .. matchOutcome.opponent .. " points")

if (matchOutcome.player > matchOutcome.opponent) then
  print("Player wins the game")
elseif (matchOutcome.player == matchOutcome.opponent) then
  print("It's a draw")
else
  print("Player loses the game")
end
