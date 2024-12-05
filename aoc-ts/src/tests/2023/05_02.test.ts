import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2023 D05 P02`, async () => {
  const ans = await solve("2023", "05", "02");
  assertEquals(ans, 6082852);
});
