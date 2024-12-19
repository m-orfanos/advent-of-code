import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D13 P02`, async () => {
  const ans = await solve("2024", "13", "02");
  assertEquals(ans, 79352015273424);
});
