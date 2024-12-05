import {
  assertEquals,
  assertExists,
} from "https://deno.land/std@0.167.0/testing/asserts.ts";

import * as day from "./day.ts";

const contents = day.readData();

Deno.bench(
  { name: "parse contents", permissions: { read: true } },
  () => {
    assertExists(day.parseContents(contents));
  },
);

const input = day.parseContents(contents);

Deno.bench(
  { name: "part 1" },
  () => {
    assertEquals(day.part1(input), day.part1Solution);
  },
);

Deno.bench(
  { name: "part 2" },
  () => {
    assertEquals(day.part2(input), day.part2Solution);
  },
);

Deno.bench(
  { name: "total" },
  () => {
    const data = day.parseContents(contents);
    assertEquals(day.part1(data), day.part1Solution);
    assertEquals(day.part2(data), day.part2Solution);
  },
);
