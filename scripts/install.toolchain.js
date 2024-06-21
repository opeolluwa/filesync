// the tool chain are mainly, Rust, and Nodejs

import shell from "shelljs";
import os from "node:os";
import { log_error } from "./utils.scripts.js";

const REQUIRED_TOOLCHAIN = ["npm", "cargo", "make", "yarn", "node"];
const SUPPORTED_PLATFORM = ["darwin", "win32", "linux"];

if (!SUPPORTED_PLATFORM.includes(os.platform())) {
  log_error(
    "Your operating system is not supported.\nConsider using one of Linux, Mac or Windows operation system"
  );
}


// install node


// install rust 


// 