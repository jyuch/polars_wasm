import { serveDir } from "https://deno.land/std@0.204.0/http/file_server.ts";
Deno.serve((request) => serveDir(request));
