/**
 * 连接相关 API
 */
import { invoke } from "@tauri-apps/api/core";
import type { Response, ConnectionConfig } from "./types";

/**
 * 连接 Redis
 */
export async function connectRedis(config: ConnectionConfig): Promise<Response<null>> {
	return await invoke<Response<null>>("connect_redis", { config });
}

/**
 * 断开 Redis 连接
 */
export async function disconnectRedis(connectionId: string): Promise<Response<null>> {
	return await invoke<Response<null>>("disconnect_redis", { connectionId });
}

