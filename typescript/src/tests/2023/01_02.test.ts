import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2023 D01 P02`, async () => {
  const ans = await solve("2023", "01", "02");
  assertEquals(ans, 55429);
});
