import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D23 P02`, async () => {
  const ans = await solve("23", "02");
  assertEquals(ans, "er,fh,fi,ir,kk,lo,lp,qi,ti,vb,xf,ys,yu");
});
