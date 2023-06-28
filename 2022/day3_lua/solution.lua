local function isInTable(table, searchValue)
  for _, v in ipairs(table) do
    if (v == searchValue) then
      return true
    end
  end
  return false
end

local function getSharedItems(c1, c2)
  local shared = {}
  for item in c1:gmatch(".") do
    if (string.match(c2, item) ~= nil) then
      if not isInTable(shared, item) then
        table.insert(shared, item)
      end
    end
  end
  return shared
end

local function getPriority(char)
  local asciiCode = string.byte(char)
  if asciiCode >= 97 and asciiCode <= 122 then
    return asciiCode - 96
  else
    return asciiCode - 65 + 27
  end
end

local sumPrioritiesOfDuplicates = 0
local file = assert(io.open("input.txt", "r"))
local lines = file:lines()

for line in lines do
  local lineLen = string.len(line)
  local rucksack = {
    firstCompartment = "",
    secondCompartment = ""
  }
  rucksack.firstCompartment = string.sub(line, 1, lineLen / 2)
  rucksack.secondCompartment = string.sub(line, lineLen / 2 + 1, lineLen)
  local sharedItems = getSharedItems(rucksack.firstCompartment, rucksack.secondCompartment)
  local priority = getPriority(sharedItems[1])
  sumPrioritiesOfDuplicates = sumPrioritiesOfDuplicates + priority
end

print("Sum", sumPrioritiesOfDuplicates)

file:close()
