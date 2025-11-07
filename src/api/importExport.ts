/**
 * 导入导出相关 API
 */
import { invoke } from "@tauri-apps/api/core";
import type { Response, KeyDetail } from "./types";

/**
 * 导出单个键
 */
export async function exportKey(
	connectionId: string,
	key: string,
): Promise<Response<KeyDetail>> {
	return await invoke<Response<KeyDetail>>("export_key", {
		connectionId,
		key,
	});
}

/**
 * 导出多个键
 */
export async function exportKeys(
	connectionId: string,
	pattern: string,
): Promise<Response<KeyDetail[]>> {
	return await invoke<Response<KeyDetail[]>>("export_keys", {
		connectionId,
		pattern,
	});
}

/**
 * 导入单个键
 */
export async function importKey(
	connectionId: string,
	keyDetail: KeyDetail,
	overwrite: boolean = false,
): Promise<Response<null>> {
	return await invoke<Response<null>>("import_key", {
		connectionId,
		keyDetail,
		overwrite,
	});
}

/**
 * 导入多个键
 */
export async function importKeys(
	connectionId: string,
	keys: KeyDetail[],
	overwrite: boolean = false,
): Promise<Response<null>> {
	return await invoke<Response<null>>("import_keys", {
		connectionId,
		keys,
		overwrite,
	});
}

