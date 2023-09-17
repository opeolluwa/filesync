import SQLite from "tauri-plugin-sqlite-api";

/** The path will be 'src-tauri/test.db', you can customize the path */
const db = await SQLite.open("./test.db");
