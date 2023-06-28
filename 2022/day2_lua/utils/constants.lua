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

return {
  handPower = handPower,
  winResponse = winResponse,
  looseResponse = looseResponse,
  drawResponse = drawResponse
}
