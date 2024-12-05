import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2023 D05 P01`, async () => {
  const ans = await solve("2023", "05", "01");
  assertEquals(ans, 3374647);
});
