import {
  Application,
  Middleware,
} from "https://deno.land/x/oak@v11.1.0/mod.ts";

import { getLogger } from "https://deno.land/std@0.165.0/log/mod.ts";

const logger = getLogger("build-report-ingestor");
const app = new Application();

export const mw: Middleware = async (ctx, next) => {
  await next();
  const body = ctx.request.body({ type: "json" });
  logger.debug(body);
  ctx.response.body = body;
};

app.use(mw);

await app.listen({ port: 8000 });
