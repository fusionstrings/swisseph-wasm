import { serve } from "https://deno.land/std@0.170.0/http/server.ts";
import { serveDir } from "https://deno.land/std@0.170.0/http/file_server.ts";

serve((req) => {
  return serveDir(req, {
    fsRoot: ".",
    showDirListing: true,
  });
}, { port: 8080 });
console.log("Listening on http://localhost:8080");
