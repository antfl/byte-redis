<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import type {
	ConnectionConfig,
	ConnectionModalAction,
	ConnectResponse,
} from "@/types";
import ConnectionModal from "@/layout/components/ConnectionModal/index.vue";

import { store } from "@/utils/storage.ts";

// 状态消息
const statusMessage: Ref<string> = ref("");
const statusClass: Ref<"success" | "error" | ""> = ref("");

// 连接状态记录
const activeConnections: Ref<Record<string, boolean>> = ref({});

// 保存的连接配置列表
const savedConnections: Ref<ConnectionConfig[]> = ref([]);

// 当前选中的连接ID
const activeConnectionId: Ref<string | null> = ref(null);

// 当前连接配置
const currentConnection = computed((): ConnectionConfig | null => {
	return (
		savedConnections.value.find((c) => c.id === activeConnectionId.value) ||
		null
	);
});

// 选择连接
function selectConnection(connectionId: string): void {
	activeConnectionId.value = connectionId;
}

// 检查连接是否已激活
function isConnected(connectionId: string): boolean {
	return !!activeConnections.value[connectionId];
}

// 删除连接
async function deleteConnection(id: string): Promise<void> {
	savedConnections.value = savedConnections.value.filter((c) => c.id !== id);
	store.set("connections", savedConnections.value);

	if (activeConnectionId.value === id) {
		activeConnectionId.value = savedConnections.value[0]?.id || null;
	}

	if (isConnected(id)) {
		try {
			await invoke("disconnect_redis", { connectionId: id });
			delete activeConnections.value[id];
		} catch (error) {
			console.error("断开连接失败:", error);
		}
	}
	message.success("连接已删除");
}

// 新建连接
const ConnectionModalRef = ref();

function newConnection(): void {
	const action: ConnectionModalAction = {
		title: "新建连接",
		onSuccess: async (formData) => {
			await submitConnection(JSON.parse(JSON.stringify(formData)));
		},
	};
	ConnectionModalRef.value?.use(action);
}

// 提交连接配置
const submitConnection = async (data: ConnectionConfig): Promise<void> => {
	if (!data.id) data.id = Date.now().toString();

	const existingIndex = savedConnections.value.findIndex(
		(c) => c.id === data.id,
	);

	if (existingIndex !== -1) {
		savedConnections.value[existingIndex] = { ...data };
	} else {
		savedConnections.value.push({ ...data });
	}

	try {
		store.set("connections", savedConnections.value);
		message.success("操作成功");
	} catch (error) {
		message.error("保存失败");
		console.error("保存连接失败:", error);
	}
};

const emit = defineEmits<{
	(
		event: "onSuccess",
		payload: {
			activeConnections: Record<string, boolean>;
			activeConnectionId: string | null;
		},
	): void;
}>();

// 编辑连接
function editConnection(row: ConnectionConfig): void {
	const action: ConnectionModalAction = {
		title: "编辑连接",
		formData: row,
		onSuccess: async (formData) => {
			await submitConnection(JSON.parse(JSON.stringify(formData)));
		},
	};
	ConnectionModalRef.value?.use(action);
}

// 连接 Redis
async function connect(): Promise<void> {
	if (!currentConnection.value) {
		message.warning("请选择或创建一个连接");
		return;
	}

	const connId = activeConnectionId.value!;
	if (isConnected(connId)) return;

	try {
		const response = await invoke<ConnectResponse>("connect_redis", {
			config: currentConnection.value,
		});

		statusMessage.value = response.message;

		if (response.success) {
			activeConnections.value[connId] = true;
			statusClass.value = "success";
			emit("onSuccess", {
				activeConnections: activeConnections.value,
				activeConnectionId: activeConnectionId.value,
			});
		} else {
			statusClass.value = "error";
		}
	} catch (error) {
		statusMessage.value = `连接失败: ${error}`;
		statusClass.value = "error";
	}
}
</script>

<template>
  <a-card>
    <div class="split-line mb-16px">
      连接管理
      <a-tag :color="statusClass">{{ statusMessage }}</a-tag>
    </div>
    <div
      v-for="conn in savedConnections"
      :key="conn.id"
      :class="{ active: activeConnectionId === conn.id }"
      @click="selectConnection(conn.id)"
    >
      <div>
        <div>{{ conn.name }}</div>
        <div>{{ conn.host }}:{{ conn.port }}</div>
      </div>
      <div>
        <a-tag :color="isConnected(conn.id) ? 'green' : 'red'">
          {{ isConnected(conn.id) ? '已连接' : '未连接' }}
        </a-tag>
        <a-space>
          <a-button size="small" @click.stop="editConnection(conn)">
            <template #icon>
              <edit-outlined/>
            </template>
          </a-button>
          <a-button size="small" danger @click.stop="deleteConnection(conn.id)">
            <template #icon>
              <delete-outlined/>
            </template>
          </a-button>
          <!-- 连接按钮 -->
          <a-button
            type="primary"
            @click="connect"
            :disabled="!conn.id || isConnected(conn.id)"
            block
          >
            {{ isConnected(conn.id) ? '已连接' : '连接 Redis' }}
          </a-button>
        </a-space>
      </div>
    </div>
    <a-button class="mb-10px" type="primary" @click="newConnection" block>
      <template #icon>
        <plus-outlined/>
      </template>
      新建连接
    </a-button>
  </a-card>

  <!-- 新建和编辑连接 -->
  <ConnectionModal ref="ConnectionModalRef"/>
</template>
