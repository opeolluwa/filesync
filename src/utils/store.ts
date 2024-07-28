import { Store } from "tauri-plugin-store-api";
const store = new Store(".send-file.dat");

/**
 * @function storeData persist application data to file system
 * @param key - the identifier
 * @param value the value to store
 * @return void
 */
export async function storeData<T>({ key, value }: { key: string; value: T }) {
  await store.set(key.trim(), { value });
}
/**
 * @function getStoredData - retrieve store data
 * @param key teh value identifier
 * @return void
 */
export async function getStoredData({ key }: { key: string }) {
  const value = await store.get(key.trim());
  return value;
}