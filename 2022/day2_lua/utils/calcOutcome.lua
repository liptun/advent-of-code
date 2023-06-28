local handPower = require("utils.constants").handPower

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

return calcOutcome
