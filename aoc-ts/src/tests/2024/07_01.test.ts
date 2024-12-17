import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D07 P01`, async () => {
  const ans = await solve("2024", "07", "01");
  assertEquals(ans, 7885693428401);
});
