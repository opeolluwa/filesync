import Database from "@tauri-apps/plugin-sql";
export const db = await Database.load("sqlite:filesync.db");

///TODO: the default settings  
