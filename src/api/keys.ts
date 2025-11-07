/**
 * 键相关 API
 */
import { invoke } from "@tauri-apps/api/core";
import type {
	Response,
	KeysListData,
	KeyDetail,
} from "./types";

/**
 * 设置键值
 */
export async function setKey(
	connectionId: string,
	key: string,
	keyType: string,
	value: any,
	ttl: number = 0,
): Promise<Response<null>> {
	return await invoke<Response<null>>("set_key", {
		connectionId,
		key,
		keyType,
		value,
		ttl,
	});
}

/**
 * 获取键值
 */
export async function getKey(
	connectionId: string,
	key: string,
): Promise<Response<string>> {
	return await invoke<Response<string>>("get_key", {
		connectionId,
		key,
	});
}

/**
 * 获取键列表
 */
export async function getKeys(
	connectionId: string,
	pattern: string = "*",
): Promise<Response<KeysListData>> {
	return await invoke<Response<KeysListData>>("get_keys", {
		connectionId,
		pattern,
	});
}

/**
 * 获取键详情
 */
export async function getKeyDetail(
	connectionId: string,
	key: string,
): Promise<Response<KeyDetail>> {
	return await invoke<Response<KeyDetail>>("get_key_detail", {
		connectionId,
		key,
	});
}

/**
 * 重命名键
 */
export async function renameKey(
	connectionId: string,
	oldKey: string,
	newKey: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("rename_key", {
		connectionId,
		oldKey,
		newKey,
	});
}

/**
 * 删除键
 */
export async function deleteKey(
	connectionId: string,
	key: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("delete_key", {
		connectionId,
		key,
	});
}

/**
 * 设置键 TTL
 */
export async function setKeyTtl(
	connectionId: string,
	key: string,
	ttl: number,
): Promise<Response<null>> {
	return await invoke<Response<null>>("set_key_ttl", {
		connectionId,
		key,
		ttl,
	});
}

/**
 * 更新哈希字段
 */
export async function updateHashField(
	connectionId: string,
	key: string,
	field: string,
	value: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("update_hash_field", {
		connectionId,
		key,
		field,
		value,
	});
}

/**
 * 删除哈希字段
 */
export async function deleteHashField(
	connectionId: string,
	key: string,
	field: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("delete_hash_field", {
		connectionId,
		key,
		field,
	});
}

/**
 * 更新列表项
 */
export async function updateListItem(
	connectionId: string,
	key: string,
	index: number,
	value: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("update_list_item", {
		connectionId,
		key,
		index,
		value,
	});
}

/**
 * 删除列表项
 */
export async function deleteListItem(
	connectionId: string,
	key: string,
	value: string,
	count: number,
): Promise<Response<null>> {
	return await invoke<Response<null>>("delete_list_item", {
		connectionId,
		key,
		value,
		count,
	});
}

/**
 * 追加列表项
 */
export async function appendListItem(
	connectionId: string,
	key: string,
	value: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("append_list_item", {
		connectionId,
		key,
		value,
	});
}

/**
 * 添加 Set 元素
 */
export async function addSetItem(
	connectionId: string,
	key: string,
	value: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("add_set_item", {
		connectionId,
		key,
		value,
	});
}

/**
 * 删除 Set 元素
 */
export async function deleteSetItem(
	connectionId: string,
	key: string,
	value: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("delete_set_item", {
		connectionId,
		key,
		value,
	});
}

/**
 * 添加 ZSet 元素
 */
export async function addZSetItem(
	connectionId: string,
	key: string,
	score: number,
	value: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("add_zset_item", {
		connectionId,
		key,
		score,
		value,
	});
}

/**
 * 删除 ZSet 元素
 */
export async function deleteZSetItem(
	connectionId: string,
	key: string,
	value: string,
): Promise<Response<null>> {
	return await invoke<Response<null>>("delete_zset_item", {
		connectionId,
		key,
		value,
	});
}

