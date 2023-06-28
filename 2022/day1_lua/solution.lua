local file = assert(io.open("input.txt", "r"))

local elves = {}
local currentElf = 1
local eof = false
repeat
  local line = file:read()
  if line == nil then
    eof = true
  else
    if line == "" then
      currentElf = currentElf + 1
    else
      local calories = tonumber(line)
      if elves[currentElf] == nil then
        elves[currentElf] = 0
      end
      elves[currentElf] = elves[currentElf] + calories
    end
  end
until (eof)

file:close()

local maxCaloriesElve = { elf = 0, calories = 0 }
for elf, calories in ipairs(elves) do
  if calories > maxCaloriesElve.calories then
    maxCaloriesElve = { elf = elf, calories = calories }
  end
end

print("The " .. maxCaloriesElve.elf .. " elf have the most snacks of " .. maxCaloriesElve.calories .. "")

-- part two

table.sort(elves, function(a, b) return a > b end)

local sumTopThreeElves = 0

for i, calories in pairs(elves) do
  if (i > 3) then
    break
  end
  sumTopThreeElves = sumTopThreeElves + calories
end

print("Top three elves have "..sumTopThreeElves.." calories of snacks")
