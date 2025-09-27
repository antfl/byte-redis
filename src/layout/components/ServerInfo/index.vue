<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, defineExpose, type Ref } from "vue";

// 定义连接状态类型
type ActiveConnections = Record<string, boolean>;

// 定义服务器统计数据
const memoryUsage = ref("0 MB");
const connections = ref(0);
const hitRate = ref(0);
const uptime = ref("0 天");

// 定义连接状态
const activeConnections: Ref<ActiveConnections> = ref({});
const activeConnectionId = ref<string | null>(null);

// 检查连接是否已激活
function isConnected(connectionId: string): boolean {
	return activeConnections.value[connectionId];
}

// 加载服务器统计数据
async function loadServerStats(): Promise<void> {
	const connectionId = activeConnectionId.value;
	if (!connectionId || !isConnected(connectionId)) return;

	try {
		// 获取内存使用情况
		const memory = await invoke<number>("get_memory_usage", { connectionId });
		memoryUsage.value = `${(memory / 1024 / 1024).toFixed(2)} MB`;

		// 获取连接数
		connections.value = await invoke<number>("get_connections", {
			connectionId,
		});

		// 获取命中率
		hitRate.value = await invoke<number>("get_hit_rate", { connectionId });

		// 获取运行时间
		const uptimeSeconds = await invoke<number>("get_uptime", { connectionId });
		const days = Math.floor(uptimeSeconds / 86400);
		const hours = Math.floor((uptimeSeconds % 86400) / 3600);
		uptime.value = `${days}天${hours}小时`;
	} catch (error) {
		console.error("加载服务器状态失败:", error);
	}
}

// 定义 load 函数的参数类型
interface LoadParams {
	_activeConnections: ActiveConnections;
	_activeConnectionId: string | null;
}

const load = (params: LoadParams): void => {
	activeConnections.value = params._activeConnections;
	activeConnectionId.value = params._activeConnectionId;
	loadServerStats();
};

defineExpose({ load });
</script>

<template>
  <a-card>
    <div class="split-line mb-16px">服务器状态</div>
    <a-row :gutter="[12, 12]">
      <a-col :span="12">
        <a-tag class="w-80%" color="cyan" :bordered="false">
          <div>内存使用</div>
          <div>{{ memoryUsage }}</div>
        </a-tag>
      </a-col>
      <a-col :span="12">
        <a-tag class="w-80%" color="processing" :bordered="false">
          <div>连接数</div>
          <div>{{ connections }}</div>
        </a-tag>
      </a-col>
      <a-col :span="12">
        <a-tag class="w-80%" color="magenta" :bordered="false">
          <div>命中率</div>
          <div>{{ hitRate }}%</div>
        </a-tag>
      </a-col>
      <a-col :span="12">
        <a-tag class="w-80%" color="blue" :bordered="false">
          <div>运行时间</div>
          <div>{{ uptime }}</div>
        </a-tag>
      </a-col>
    </a-row>
  </a-card>
</template>
