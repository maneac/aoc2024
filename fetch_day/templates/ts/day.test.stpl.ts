import { assertEquals } from "https://deno.land/std@0.167.0/testing/asserts.ts";

import * as day from "./day.ts";

const contents = day.readData();

Deno.test(
  "parse contents",
  async (t) => {
    const cases = {
      "example": {
        input: ``,
        expected: exampleData(),
      },
    };

    for (const [name, test] of Object.entries(cases)) {
      await t.step(name, () => {
        assertEquals(day.parseContents(test.input), test.expected);
      });
    }
  },
);

Deno.test(
  { name: "part 1", permissions: { read: true } },
  async (t) => {
    const cases = {
      "example": {
        input: exampleData(),
        expected: 0,
      },
      "actual": {
        input: day.parseContents(contents),
        expected: day.part1Solution,
      },
    };

    for (const [name, test] of Object.entries(cases)) {
      await t.step(name, () => {
        assertEquals(day.part1(test.input), test.expected);
      });
    }
  },
);

Deno.test(
  { name: "part 2", permissions: { read: true } },
  async (t) => {
    const cases = {
      "example": {
        input: exampleData(),
        expected: 0,
      },
      "actual": {
        input: day.parseContents(contents),
        expected: day.part2Solution,
      },
    };

    for (const [name, test] of Object.entries(cases)) {
      await t.step(name, () => {
        assertEquals(day.part2(test.input), test.expected);
      });
    }
  },
);

function exampleData(): day.Input {
  throw new Error("unimplemented");
}
