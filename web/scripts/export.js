import shell from "shelljs";
import { existsSync } from "node:fs";

const PWD = shell.pwd().stdout;

if (!existsSync(`${PWD}/dist`)) {
  shell.exec(`node scripts/build`);
}

shell.cp("-r", "dist", "../desktop/core/static-test");