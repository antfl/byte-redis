/**
 * 统一响应实体 - 所有接口都使用此实体返回
 * T 为泛型，可以是任意类型：数字、字符串、null、集合、对象等
 */
export interface Response<T = any> {
	success: boolean;
	message: string;
	data?: T;
}

/**
 * 连接配置
 */
export interface ConnectionConfig {
	id: string;
	name: string;
	host: string;
	port: number;
	username?: string;
	password?: string;
	db?: number;
}

/**
 * 键信息
 */
export interface KeyInfo {
	key: string;
	type: string;
}

/**
 * 键列表数据
 */
export interface KeysListData {
	keys: KeyInfo[];
	total: number;
}

/**
 * 键详情
 */
export interface KeyDetail {
	key: string;
	type: string;
	ttl: number;
	size: number;
	create_time: string;
	value: any;
}

/**
 * 数据库键数量
 */
export interface DbKeyCount {
	db_index: number;
	key_count: number;
}

/**
 * Redis 服务器信息
 */
export interface RedisServerInfo {
	memory_usage: number;
	maxmemory: number;
	connections: number;
	hit_rate: number;
	uptime: number;
	total_keys: number;
	ops_per_sec: number;
	used_cpu: number;
	role: string;
	version: string;
	persistence: string;
	clients_blocked: number;
	keys_evicted: number;
	keys_expired: number;
	replication_status: string;
	mem_fragmentation_ratio: number;
	aof_size: number;
	rdb_last_save: number;
	connected_slaves: number;
}

