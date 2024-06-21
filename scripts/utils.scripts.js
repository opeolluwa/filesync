import chalk from "chalk";

export const log_error = (
  message = "the command does not exit successfully!\nPlease retry"
) => {
  console.error(chalk.red(message));
};

export const log_info = (message) => {
  console.error(chalk.blue(message));
};

export const log_success = (message = "the command exited successfully!") => {
  console.error(chalk.green(message));
};

export const log_warning = (message) => {
  console.error(chalk.yellow(message));
};
