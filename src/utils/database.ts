import SQLite from "tauri-plugin-sqlite-api";

/** The path will be 'src-tauri/test.db', you can customize the path */
const database = await SQLite.open("./test.db");
/** if your file named 'test.sqlite' and it's in current directory './
await SQLite.open("./test.db");

// /** execute SQL */
// await db.execute(`
//     CREATE TABLE users (name TEXT, age INTEGER);
//     INSERT INTO users VALUES ('Alice', 42);
//     INSERT INTO users VALUES ('Bob', 69);
// `);

// /** execute SQL with params */
// await db.execute("INSERT INTO users VALUES (?1, ?2)", ["Jack", 18]);

// /** batch execution SQL with params */
// await db.execute("INSERT INTO users VALUES (?1, ?2)", [
//   ["Allen", 20],
//   ["Barry", 16],
//   ["Cara", 28],
// ]);

// /** select count */
// const rows = await db.select<Array<{ count: number }>>(
//   "SELECT COUNT(*) as count FROM users"
// );

// /** select with param */
// const rows = await db.select<Array<{ name: string }>>(
//   "SELECT name FROM users WHERE age > ?",
//   [20]
// );

// /** select with params, you can use ? or $1 .. $n */
// const rows = await db.select<Array<any>>(
//   "SELECT * FROM users LIMIT $1 OFFSET $2",
//   [10, 0]
// );

// /** close sqlite database */
// const isClosed = await db.close();
