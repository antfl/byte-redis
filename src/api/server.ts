/**
 * 服务器相关 API
 */
import { invoke } from "@tauri-apps/api/core";
import type { Response, RedisServerInfo } from "./types";

/**
 * 获取 Redis 服务器信息
 */
export async function getRedisServerInfo(
	connectionId: string,
): Promise<Response<RedisServerInfo>> {
	return await invoke<Response<RedisServerInfo>>("get_redis_server_info", {
		connectionId,
	});
}

