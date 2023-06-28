import fs from "fs/promises";

const loadInputData = async () => {
  return await fs.readFile("./input.txt", "utf-8");
};

type Elf = {
  name: string;
  calories: number;
};

const main = async () => {
  const rawData = await loadInputData();
  const data = rawData.split("\n");

  const elves: Elf[] = data.reduce((acc, curr) => {
    const draftAcc = [...acc];
    if (draftAcc.length === 0 || curr === "") {
      const newElf: Elf = {
        name: `Elf ${draftAcc.length + 1}`,
        calories: 0,
      };
      draftAcc.push(newElf);
    }
    if (curr !== "") {
      draftAcc[draftAcc.length - 1].calories += parseInt(curr);
    }
    return draftAcc;
  }, [] as Elf[]);

  elves.sort((a, b) => (a.calories < b.calories ? 1 : -1));

  const topThreeElves = elves.reduce((acc, curr) => {
    const draftAcc = [...acc];
    if (draftAcc.length < 3) {
      draftAcc.push(curr);
    }

    return draftAcc;
  }, [] as Elf[]);

  const topElf = topThreeElves[0];
  const namesOfThreeTopElves = topThreeElves.reduce((acc, curr) => {
    return `${acc}${acc.length ? ", " : ""}${curr.name}`;
  }, "" as string);
  const sumOfThreeTopElvesCalories = topThreeElves.reduce(
    (acc, curr) => acc + curr.calories,
    0
  );

  console.log("--- Part One ---");
  console.log(
    `The most snacks have ${topElf.name}. His snacks have ${topElf.calories} calories`
  );
  console.log("--- Part Two ---");
  console.log(
    `The top three elves ${namesOfThreeTopElves} carries ${sumOfThreeTopElvesCalories} calories of snacks`
  );
};

main();
