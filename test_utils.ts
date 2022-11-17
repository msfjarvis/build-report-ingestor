import {
  NativeRequest,
  NativeRequestOptions,
} from "https://deno.land/x/oak@v11.1.0/http_server_native_request.ts";

export const createMockNativeRequest = (
  url = "http://localhost/index.html",
  requestInit: RequestInit = {},
  options?: NativeRequestOptions,
) => {
  // deno-lint-ignore no-explicit-any
  const request: globalThis.Request = new (globalThis as any).Request(
    url,
    requestInit,
  );

  return new NativeRequest(
    {
      request,
      async respondWith(r) {
        await r;
      },
    },
    options,
  );
};
