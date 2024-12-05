import * as day from "./day.ts";

const contents = day.readData();
const data = day.parseContents(contents);
console.log("Part 1: ", day.part1(data));
console.log("Part 2: ", day.part2(data));
