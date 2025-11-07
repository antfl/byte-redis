/**
 * 数据库相关 API
 */
import { invoke } from "@tauri-apps/api/core";
import type { Response, DbKeyCount } from "./types";

/**
 * 获取数据库数量
 */
export async function getDbCount(connectionId: string): Promise<Response<number>> {
	return await invoke<Response<number>>("get_db_count", {
		connectionId,
	});
}

/**
 * 获取数据库键数量
 */
export async function getDbKeyCount(
	connectionId: string,
	dbIndex: number,
): Promise<Response<number>> {
	return await invoke<Response<number>>("get_db_key_count", {
		connectionId,
		dbIndex,
	});
}

/**
 * 批量获取所有数据库的键数量
 */
export async function getAllDbKeyCounts(
	connectionId: string,
	dbCount: number,
): Promise<Response<DbKeyCount[]>> {
	return await invoke<Response<DbKeyCount[]>>("get_all_db_key_counts", {
		connectionId,
		dbCount,
	});
}

/**
 * 切换数据库
 */
export async function selectDb(
	connectionId: string,
	dbIndex: number,
): Promise<Response<null>> {
	return await invoke<Response<null>>("select_db", {
		connectionId,
		dbIndex,
	});
}

