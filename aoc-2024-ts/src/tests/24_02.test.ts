import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D24 P02`, async () => {
  const ans = await solve("24", "02");
  assertEquals(ans, "djg,dsd,hjm,mcq,sbg,z12,z19,z37");
});
