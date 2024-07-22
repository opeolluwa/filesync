import shell from "shelljs";
import { log_info, log_success } from "./utils.scripts.js";

// install the repo deps
log_info("Initializing...");

shell.exec("npm install --save")
const sources = ["web", "mobile", "desktop", "homepage"];

sources.forEach((source) => {
  log_info(`Setting up ${source.toUpperCase()} ...`);
  shell.exec(`
    cd ${source} && npm install --save
    `);
  log_success(`${source.toUpperCase()} setup complete\n`);
});
