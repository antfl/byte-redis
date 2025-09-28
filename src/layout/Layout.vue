<script setup>
import ConnectionManager from "@/layout/components/ConnectionManager/index.vue";
import DatabaseList from "@/layout/components/DatabaseList/index.vue";
import ServerInfo from "@/layout/components/ServerInfo/index.vue";
import RedisDataTable from "@/layout/components/RedisDataTable/index.vue";
import TitleBar from "@/components/TitleBar/index.vue";

// 创建存储实例
import { store } from "@/utils/storage.ts";

// 连接管理相关
const savedConnections = ref([]);
const activeConnectionId = ref(null);

// Redis 连接相关
const activeConnections = ref({});

const connectSuccess = (data) => {
	activeConnections.value = data.activeConnections;
	activeConnectionId.value = data.activeConnectionId;
	// 加载数据库信息
	loadDatabaseInfo();
	// 加载键列表
	loadKeys();
	// 加载服务器状态
	loadServerInfo();
};

onMounted(async () => {
	// 加载保存的连接
	await loadConnections();
	// 如果有保存的连接，选择第一个
	if (savedConnections.value.length > 0) {
		activeConnectionId.value = savedConnections.value[0].id;
	}
});

// 检查连接是否已激活
function isConnected(connectionId) {
	return activeConnections.value[connectionId] === true;
}

// 加载保存的连接
async function loadConnections() {
	try {
		const loaded = await store.get("connections");
		if (loaded) {
			savedConnections.value = loaded;
		}
	} catch (error) {
		console.error("加载连接失败:", error);
	}
}

/** 服务信息相关 */
const ServerInfoRef = ref();
const loadServerInfo = () => {
	ServerInfoRef.value.load({
		_activeConnections: activeConnections.value,
		_activeConnectionId: activeConnectionId.value,
	});
};

/** 数据库相关 */
const DatabaseListRef = ref();

async function loadDatabaseInfo() {
	DatabaseListRef.value.load({
		_activeConnections: activeConnections.value,
		_activeConnectionId: activeConnectionId.value,
	});
}

/** Redis 数据相关 */
const RedisDataTableRef = ref();

async function loadKeys() {
	if (!activeConnectionId.value || !isConnected(activeConnectionId.value))
		return;
	RedisDataTableRef.value.load({
		_activeConnections: activeConnections.value,
		_activeConnectionId: activeConnectionId.value,
	});
}
</script>

<template>
  <a-layout>
    <a-layout-header class="h-[var(--title-bar-height)]!">
      <TitleBar/>
    </a-layout-header>
    <a-layout class="h-[calc(100vh_-_var(--title-bar-height))] overflow-hidden">
      <a-layout-sider theme="light" width="280">
        <!-- 连接管理 -->
        <ConnectionManager @on-success="connectSuccess"/>

        <!-- 数据库 -->
        <DatabaseList ref="DatabaseListRef" @on-success="loadKeys"/>

        <!-- 服务器信息 -->
        <ServerInfo ref="ServerInfoRef"/>
      </a-layout-sider>
      <a-layout-content>
        <!-- Redis 数据 -->
        <RedisDataTable ref="RedisDataTableRef"/>
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>
