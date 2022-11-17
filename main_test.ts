import {
  assert,
  assertEquals,
} from "https://deno.land/std@0.164.0/testing/asserts.ts";
import { testing } from "https://deno.land/x/oak@v11.1.0/mod.ts";
import { Request } from "https://deno.land/x/oak@v11.1.0/mod.ts";
import { mw } from "./main.ts";
import "./test_utils.ts";
import { createMockNativeRequest } from "./test_utils.ts";

Deno.test({
  name: "Example test",
  async fn() {
    const ctx = testing.createMockContext({
      path: "/",
      method: "POST",
      params: {},
    });

    const body = JSON.stringify({ "name": "Deno" });
    const request = new Request(
      createMockNativeRequest("https://localhost", {
        body,
        method: "POST",
        headers: {},
      }),
    );
    ctx.request = request;
    assert(ctx.request.hasBody);
    const next = testing.createMockNext();
    await mw(ctx, next);
    assertEquals(ctx.response.body, body);
  },
});
