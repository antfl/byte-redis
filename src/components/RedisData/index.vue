<script setup lang="ts">
import { formatTTL } from "@/utils/format.ts";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import AddKeyModal from "@/components/AddKeyModal/index.vue";
import ViewKeyModal from "@/components/ViewKeyModal/index.vue";
import type { KeyType } from "@/types/redis.ts";

// 定义类型
interface RedisKey {
	key: string;
	type: KeyType;
	ttl: number;
	size: number;
}

interface ConnectionStatus {
	[connectionId: string]: boolean;
}

interface LoadParams {
	_activeConnectionId: string | null;
	_activeConnections: ConnectionStatus;
}

interface KeysResponse {
	success: boolean;
	keys: RedisKey[];
	total: number;
	message?: string;
}

interface TableColumn {
	title: string;
	dataIndex: string;
	key: string;
	ellipsis?: boolean;
	width?: number;
	sorter?: (a: RedisKey, b: RedisKey) => number;
	filters?: { text: string; value: string }[];
	onFilter?: (value: string, record: RedisKey) => boolean;
}

// 响应式状态
const activeConnections = ref<ConnectionStatus>({});
const activeConnectionId = ref<string | null>(null);
const currentDb = ref<number>(0);
const keys = ref<RedisKey[]>([]);
const loading = ref<boolean>(false);
const searchPattern = ref<string>("*");
const debounceTimer = ref<ReturnType<typeof setTimeout> | null>(null);
const lastLoadedConnectionId = ref<string | null>(null);

// 表格列定义
const columns = ref<TableColumn[]>([
	{
		title: "键名",
		dataIndex: "key",
		key: "key",
		ellipsis: true,
		width: 400,
		sorter: (a: RedisKey, b: RedisKey) => a.key.localeCompare(b.key),
	},
	{
		title: "类型",
		dataIndex: "type",
		key: "type",
	},
	{
		title: "TTL",
		dataIndex: "ttl",
		key: "ttl",
		sorter: (a: RedisKey, b: RedisKey) => a.ttl - b.ttl,
	},
]);

// 分页配置
const pagination = reactive({
	current: 1,
	pageSize: 20,
	total: 0,
	showSizeChanger: true,
	pageSizeOptions: ["10", "20", "50", "100"],
});

// 检查连接是否已激活
function isConnected(connectionId: string): boolean {
	return activeConnections.value[connectionId] === true;
}

// 获取键类型对应的颜色
function getTypeColor(type: KeyType): string {
	const typeColors: Record<KeyType, string> = {
		string: "blue",
		hash: "green",
		list: "orange",
		set: "purple",
		zset: "red",
		stream: "cyan",
		bitmap: "processing",
		hyperloglog: "magenta",
		geospatial: "volcano",
	};
	return typeColors[type] || "gray";
}

// 模态框引用
const ViewKeyModalRef = ref();
const AddKeyModalRef = ref();

// 显示键详情
async function showKeyDetail(record: RedisKey): Promise<void> {
	if (!activeConnectionId.value || !ViewKeyModalRef.value) return;

	ViewKeyModalRef.value.use({
		record,
		connectionId: activeConnectionId.value,
	});
}

// 删除键
async function deleteKey(record: RedisKey): Promise<void> {
	if (!activeConnectionId.value || !isConnected(activeConnectionId.value))
		return;

	try {
		await invoke("delete_key", {
			connectionId: activeConnectionId.value,
			key: record.key,
		});
		message.success(`键 "${record.key}" 已删除`);

		// 从本地状态中移除键
		keys.value = keys.value.filter((k) => k.key !== record.key);
		pagination.total -= 1;

		// 如果当前页没有数据且不是第一页，则跳转到上一页
		if (keys.value.length === 0 && pagination.current > 1) {
			pagination.current -= 1;
			await loadKeys();
		}
	} catch (error) {
		message.error(`删除键失败: ${error}`);
	}
}

// 加载键列表
async function loadKeys(): Promise<void> {
	if (!activeConnectionId.value || !isConnected(activeConnectionId.value)) {
		keys.value = [];
		pagination.total = 0;
		return;
	}

	loading.value = true;
	try {
		const offset = (pagination.current - 1) * pagination.pageSize;

		const response = await invoke<KeysResponse>("get_keys_with_details", {
			connectionId: activeConnectionId.value,
			pattern: searchPattern.value || "*",
			offset: offset,
			limit: pagination.pageSize,
		});

		if (response.success) {
			keys.value = response.keys;
			pagination.total = response.total || 0;

			// 如果当前页没有数据且不是第一页，则跳转到上一页
			if (keys.value.length === 0 && pagination.current > 1) {
				pagination.current -= 1;
				await loadKeys();
			}
		} else {
			message.error(response.message || "加载键列表失败");
			keys.value = [];
			pagination.total = 0;
		}
	} catch (error) {
		message.error(`加载键列表失败: ${error}`);
		keys.value = [];
		pagination.total = 0;
	} finally {
		loading.value = false;
	}
}

// 搜索处理
function handleSearch(): void {
	if (debounceTimer.value) {
		clearTimeout(debounceTimer.value);
	}

	debounceTimer.value = setTimeout(() => {
		pagination.current = 1;
		loadKeys();
	}, 300);
}

// 显示添加键模态框
function showAddKeyModal(): void {
	if (!activeConnectionId.value || !AddKeyModalRef.value) return;

	AddKeyModalRef.value.use({
		connectionId: activeConnectionId.value,
		onSuccess: async (newKey: RedisKey) => {
			try {

				// 更新本地状态
				keys.value = [newKey, ...keys.value];
				pagination.total += 1;

				// 如果当前页已满，跳转到第一页
				if (keys.value.length > pagination.pageSize) {
					pagination.current = 1;
					await loadKeys();
				}

				message.success("键添加成功");
			} catch (error) {
				message.error(`添加键失败: ${error}`);
			}
		},
	});
}

// 处理表格分页变化
function handleTableChange(pag: { current: number; pageSize: number }): void {
	pagination.current = pag.current;
	pagination.pageSize = pag.pageSize;
	loadKeys();
}

// 加载组件数据
const load = async ({
	_activeConnectionId,
	_activeConnections,
}: LoadParams): Promise<void> => {
	activeConnectionId.value = _activeConnectionId;
	activeConnections.value = _activeConnections;

	// 只有在连接发生变化时才重新加载
	if (lastLoadedConnectionId.value !== _activeConnectionId) {
		pagination.current = 1;
		searchPattern.value = "*";
		lastLoadedConnectionId.value = _activeConnectionId;
		await loadKeys();
	}
};

// 监听连接变化
watch(activeConnectionId, (newId) => {
	if (newId && lastLoadedConnectionId.value !== newId) {
		pagination.current = 1;
		loadKeys();
	}
});

// 组件挂载时加载当前数据库信息
onMounted(async () => {
	if (activeConnectionId.value) {
		try {
			const dbInfo = await invoke<{ db: number }>("get_current_db", {
				connectionId: activeConnectionId.value,
			});
			currentDb.value = dbInfo.db || 0;
		} catch (error) {
			console.error("获取当前数据库失败:", error);
		}
	}
});

defineExpose({
	load,
});
</script>

<template>
  <div>
      <a-tabs
        class="mb-16px"
        :tabBarStyle="{
        margin: '0',
        height: '30px',
        backgroundColor: '#fff',
      }">
        <a-tab-pane key="1">
          <template #tab>
            <BlockOutlined class="mr-3px!"/>
            图形界面
          </template>
        </a-tab-pane>
        <a-tab-pane key="2">
          <template #tab>
            <CodeOutlined class="mr-3px!"/>
            命令行
          </template>
        </a-tab-pane>
      </a-tabs>
      <div class="flex items-center  mb-16px">
        <a-form layout="inline" class="">
          <a-form-item class="">
            <a-input
              v-model:value="searchPattern"
              placeholder="搜索键 (支持通配符 *)"
              @search="handleSearch"
              @input="handleSearch"
              allow-clear
            />
          </a-form-item>
        </a-form>

        <div class="flex gap-10px">
          <a-button type="primary" @click="loadKeys" :loading="loading">
            <template #icon>
              <reload-outlined/>
            </template>
            搜索
          </a-button>
          <a-button type="primary" @click="showAddKeyModal" :disabled="!activeConnectionId">
            <template #icon>
              <plus-outlined/>
            </template>
            添加键
          </a-button>
        </div>
      </div>
      <a-table
        bordered
        :columns="columns"
        :data-source="keys"
        :pagination="pagination"
        :loading="loading"
        size="small"
        :scroll="{ x: 'auto', y: 500 }"
        @change="handleTableChange"
        :rowKey="(record: RedisKey) => record.key"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.dataIndex === 'key'">
            <div class="flex justify-between">
              <a-tooltip :title="record.key" placement="top">
                <span class="cursor-pointer hover:text-blue-500" @click="showKeyDetail(record)">
                  {{ record.key }}
                </span>
              </a-tooltip>
              <div>
                <a-tooltip title="查看详情">
                  <a-button size="small" type="link" @click="showKeyDetail(record)">
                    详情
                  </a-button>
                </a-tooltip>
                <a-tooltip title="删除键">
                  <a-popconfirm
                    title="确定要删除这个键吗？"
                    ok-text="确定"
                    cancel-text="取消"
                    @confirm="deleteKey(record)"
                  >
                    <a-button size="small" type="link" danger>
                      删除
                    </a-button>
                  </a-popconfirm>
                </a-tooltip>
              </div>
            </div>
          </template>

          <template v-if="column.dataIndex === 'type'">
            <a-tag :color="getTypeColor(record.type)" :bordered="false">
              {{ record.type }}
            </a-tag>
          </template>

          <template v-if="column.dataIndex === 'ttl'">
            <span v-if="record.ttl === -1">永不过期</span>
            <span v-else>{{ formatTTL(record.ttl) }}</span>
          </template>
        </template>

        <template #emptyText>
          <div class="py-20px text-center">
            <a-empty description="没有找到匹配的键"/>
          </div>
        </template>
      </a-table>

    <!-- 添加键模态框 -->
    <AddKeyModal ref="AddKeyModalRef"/>

    <!-- 键详情模态框 -->
    <ViewKeyModal ref="ViewKeyModalRef"/>
  </div>
</template>
