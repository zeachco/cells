/// <reference lib="dom" />
/// <reference lib="dom.iterable" />
import * as ReactDOM from "react-dom/client";

import { App } from "./App.tsx";

const rootEl = document.createElement("div");
document.body.appendChild(rootEl);

const root = ReactDOM.createRoot(rootEl);
root.render(<App />);

console.log(`Listening on http://localhost:${server.port} ...`);
