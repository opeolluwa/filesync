import shell from "shelljs";

shell.rm("-r", "../desktop/core/static");
shell.exec(`node scripts/build`);
shell.cp("-r", "dist", "../desktop/core/static");
