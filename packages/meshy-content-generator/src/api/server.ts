import { buildServer } from "./app.js";

const port = Number(process.env.PORT ?? 5177);

const app = buildServer();
app.listen({ port, host: "0.0.0.0" });
