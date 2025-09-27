/**
 * Redis 键支持的数据类型
 */
export type KeyType =
	| "string" // 字符串类型
	| "hash" // 哈希类型
	| "list" // 列表类型
	| "set" // 集合类型
	| "zset" // 有序集合类型
	| "stream" // 流类型（Redis 5.0 +）
	| "bitmap" // 位图类型
	| "hyperloglog" // HyperLogLog 类型
	| "geospatial"; // 地理空间类型

/**
 * 键值对数据结构
 */
export interface KeyValuePair {
	key: string;
	value: string;
	type: KeyType;
	ttl: number; // 过期时间（秒），-1 表示永不过期
}

/**
 * Redis 命令响应类型
 */
export type RedisResponse =
	| string
	| number
	| boolean
	| null
	| string[]
	| { [key: string]: string };

/**
 * Redis 连接配置
 */
export interface RedisConnection {
	host: string;
	port: number;
	username?: string;
	password?: string;
	db?: number;
	name?: string; // 连接名称
	tls?: boolean; // 是否使用 TLS
}

/**
 * Redis 服务器信息
 */
export interface RedisServerInfo {
	version: string;
	mode: string; // standalone, cluster, sentinel
	os: string;
	memory: {
		total: number;
		used: number;
		peak: number;
	};
	clients: {
		connected: number;
		max: number;
	};
}
