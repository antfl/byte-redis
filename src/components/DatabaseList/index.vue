<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";

// 定义数据库类型
interface DatabaseInfo {
	index: number;
	keyCount: number;
}

// 定义连接状态类型
type ActiveConnections = Record<string, boolean>;

// 初始化数据库列表
const databases = ref<DatabaseInfo[]>(
	Array.from({ length: 16 }, (_, i) => ({
		index: i,
		keyCount: 0,
	})),
);

const currentDb = ref<number>(0);
const activeConnections = ref<ActiveConnections>({});
const activeConnectionId = ref<string | null>(null);

const emit = defineEmits<{
	(event: "onSuccess"): void;
}>();

// 检查连接是否已激活
function isConnected(connectionId: string): boolean {
	return activeConnections.value[connectionId];
}

async function selectDb(dbIndex: number): Promise<void> {
	const connectionId = activeConnectionId.value;
	if (!connectionId || !isConnected(connectionId)) return;

	try {
		await invoke("select_db", { connectionId, dbIndex });
		currentDb.value = dbIndex;
		emit("onSuccess");
	} catch (error) {
		message.error(
			`切换数据库失败: ${error instanceof Error ? error.message : String(error)}`,
		);
	}
}

// 定义 load 函数的参数类型
interface LoadParams {
	_activeConnectionId: string;
	_activeConnections: ActiveConnections;
}

const load = async ({
	_activeConnectionId,
	_activeConnections,
}: LoadParams): Promise<void> => {
	activeConnectionId.value = _activeConnectionId;
	activeConnections.value = _activeConnections;

	if (!_activeConnectionId || !isConnected(_activeConnectionId)) return;

	try {
		// 获取数据库数量
		const dbCount = await invoke<number>("get_db_count", {
			connectionId: _activeConnectionId,
		});

		// 更新数据库列表
		databases.value = Array.from({ length: dbCount }, (_, i) => ({
			index: i,
			keyCount: databases.value[i]?.keyCount || 0,
		}));

		// 并行获取所有数据库的键数量
		const keyCountPromises = databases.value.map((db) =>
			invoke<number>("get_db_key_count", {
				connectionId: _activeConnectionId,
				dbIndex: db.index,
			}),
		);

		const keyCounts = await Promise.all(keyCountPromises);
		databases.value = databases.value.map((db, i) => ({
			...db,
			keyCount: keyCounts[i],
		}));
	} catch (error) {
		console.error("加载数据库信息失败:", error);
		message.error(
			`加载数据库信息失败: ${error instanceof Error ? error.message : String(error)}`,
		);
	}
};

defineExpose({ load });
</script>

<template>
  <div class="ml-30px">
    <a-select :bordered="false" placeholder="数据库" @change="selectDb">
      <a-select-option
        v-for="(item, index) in databases"
        :value="item.index"
        :key="index"
      >
          DB {{ item.index + 1 }}
          <span :class="`${currentDb === item.index ? 'color-blue' : 'color-#1e1f22'}`">
            [{{ item.keyCount }}]
          </span>
      </a-select-option>
    </a-select>
  </div>
</template>
