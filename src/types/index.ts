export interface ConnectionConfig {
	id: string;
	name: string;
	host: string;
	port: number;
	username?: string;
	password?: string;
	database?: number;
	ssl?: boolean;
	timeout?: number;
	[key: string]: any; // 允许其他扩展属性
}

export interface ConnectResponse {
	success: boolean;
	message: string;
	latency?: number;
}

export interface ConnectionModalAction {
	title: string;
	formData?: ConnectionConfig;
	onSuccess: (data: { formData: Ref<ConnectionConfig> }) => Promise<void>;
}
