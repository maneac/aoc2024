export const part1Solution = 0;
export const part2Solution = 0;

export type Input = {};

export function readData(): string {
  return Deno.readTextFileSync("./data/<%= &self.package_name %>.txt")
    .trim();
}

export function parseContents(contents: string): Input {
  throw new Error("unimplemented");
}

export function part1(data: Input): number {
  throw new Error("unimplemented");
}

export function part2(data: Input): number {
  throw new Error("unimplemented");
}
