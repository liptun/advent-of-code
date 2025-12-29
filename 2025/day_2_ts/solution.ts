import fs from "fs/promises";

const loadInputData = async () => {
  return await fs.readFile("./input.txt", "utf-8");
};

const parseProductIdList = (productListRaw: string) => {
    return productListRaw.split(",").map((rangeString) => {
        return rangeString.split("-").map((rangeValue) => parseInt(rangeValue));
    });
};

const createIdArrayByRange = (start: number, end: number): number[] => {
    let range: number[] = [];

    for (let i = start; i <= end; i++) {
        range.push(i);
    }

    return range;
};

const isValidId = (id: string): boolean => {
    if (id.length % 2 > 0) {
        return true;
    }

    const left = id.substring(0, id.length / 2);
    const right = id.substring(id.length / 2, id.length);

    return left !== right;
};

const createRangeTuple = (range: number[]): [number, number] | null => {
    if (range.length !== 2) {
        return null;
    }

    const start = range[0];
    const end = range[1];

    if (start === undefined || end === undefined) {
        return null;
    }

    return [start, end];
};



const main = async () => {

    const input = await loadInputData();

    console.log(input);

    const ids = parseProductIdList(input)
        .map((range) => {
            const rangeTuple = createRangeTuple(range);

            if (rangeTuple === null) {
                return null;
            }
            const [start, end] = rangeTuple;

            return createIdArrayByRange(start, end);
        })
        .filter((range) => range !== null)
        .flat();

    const invalidIds = ids.filter((id) => !isValidId(id.toString()));
    const invalidIdsSum = invalidIds.reduce((acc, curr) => {
        return acc + curr;
    }, 0);

    console.log(`Sum of all invalid id's is ${invalidIdsSum}`);
};

main();
