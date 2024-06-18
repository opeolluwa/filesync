import shell from "shelljs";

shell.exec(`node scripts/build`);
shell.cp("-r", "dist", "../desktop/core/static-test");
