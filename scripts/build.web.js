// build the web static assets and export

import shell from "shelljs";

import { log_success } from "./utils.scripts.js";

shell.exec("cd web && npm run export")

log_success("Web static files exported successfully!")