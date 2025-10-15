<script setup lang="ts">
import { formatTTL } from "@/utils/format.ts";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";

// 定义类型
interface ConnectionStatus {
	[connectionId: string]: boolean;
}

interface KeyRecord {
	key: string;
	type?: string;
	ttl?: number;
}

interface UseParams {
	_activeConnections: ConnectionStatus;
	_activeConnectionId: string | null;
	connectionId: string | null;
	record: KeyRecord;
}

// 响应式状态
const activeConnectionId = ref<string | null>(null);
const activeConnections = ref<ConnectionStatus>({});
const visible = ref(false);
const currentKey = ref<string>("");
const keyValue = ref<string>("");
const currentTTL = ref<number>(-1);
const currentExpire = ref<string>("");
const newTTLValue = ref<number>(-1);
const activeDetailTab = ref<"value" | "ttl">("value");

// 计算属性：格式化后的 TTL 显示
const formattedTTL = computed(() => formatTTL(currentTTL.value));

// 更新键的 TTL
async function updateTTL(): Promise<void> {
	if (!activeConnectionId.value || !isConnected(activeConnectionId.value))
		return;

	try {
		await invoke("set_key_ttl", {
			connectionId: activeConnectionId.value,
			key: currentKey.value,
			ttl: newTTLValue.value,
		});
		message.success("TTL 更新成功");
		currentTTL.value = newTTLValue.value;
	} catch (error) {
		message.error(`更新 TTL 失败: ${error}`);
	}
}

// 移除键的 TTL
async function removeTTL(): Promise<void> {
	if (!activeConnectionId.value || !isConnected(activeConnectionId.value))
		return;

	try {
		await invoke("set_key_ttl", {
			connectionId: activeConnectionId.value,
			key: currentKey.value,
			ttl: -1,
		});
		message.success("TTL 已移除");
		currentTTL.value = -1;
		newTTLValue.value = -1;
	} catch (error) {
		message.error(`移除 TTL 失败: ${error}`);
	}
}

// 保存键值
async function saveKeyValue(): Promise<void> {
	if (!activeConnectionId.value || !isConnected(activeConnectionId.value))
		return;

	try {
		await invoke("set_key", {
			connectionId: activeConnectionId.value,
			key: currentKey.value,
			value: keyValue.value,
		});
		message.success("键值保存成功");
	} catch (error) {
		message.error(`保存键值失败: ${error}`);
	}
}

// 检查连接是否已激活
function isConnected(connectionId: string): boolean {
	return activeConnections.value[connectionId] === true;
}

// 初始化并显示模态框
const use = async (params: UseParams): Promise<void> => {
	activeConnections.value = params._activeConnections;
	activeConnectionId.value = params._activeConnectionId;
	visible.value = true;

	if (!activeConnectionId.value || !isConnected(activeConnectionId.value))
		return;

	currentKey.value = params.record.key;

	try {
		const value = await invoke<string>("get_key", {
			connectionId: activeConnectionId.value,
			key: params.record.key,
		});

		const ttl = await invoke<number>("get_key_ttl", {
			connectionId: activeConnectionId.value,
			key: params.record.key,
		});

		keyValue.value = value;
		currentTTL.value = ttl;

		// 计算过期时间
		if (ttl > 0) {
			const expireDate = new Date(Date.now() + ttl * 1000);
			currentExpire.value = expireDate.toLocaleString();
		} else {
			currentExpire.value = "永不过期";
		}
	} catch (error) {
		message.error(`获取键详情失败: ${error}`);
	}
};

defineExpose({
	use,
});
</script>

<template>
  <a-modal
    v-model:open="visible"
    :title="'键详情: ' + currentKey"
    width="800px"
    class="byte-modal"
    :centered="true"
    :footer="null"
  >
    <a-tabs v-model:activeKey="activeDetailTab">
      <a-tab-pane key="value" tab="值">
        <a-textarea
          v-model:value="keyValue"
          :rows="10"
          placeholder="键值内容"
        />
      </a-tab-pane>
      <a-tab-pane key="ttl" tab="TTL">
        <div>
          <div>
            <div>
              <span>剩余时间:</span>
              <span>{{ formattedTTL }}</span>
            </div>
            <div>
              <span>过期时间:</span>
              <span>{{ currentExpire }}</span>
            </div>
          </div>
          <div class="flex gap-10px">
            <a-input-number
              v-model:value="newTTLValue"
              :min="-1"
              :step="1"
              placeholder="设置新 TTL（秒）"
            />
            <a-button type="primary" @click="updateTTL">更新TTL</a-button>
            <a-button @click="removeTTL">移除TTL</a-button>
          </div>
        </div>
      </a-tab-pane>
    </a-tabs>
    <div class="mt-16px flex justify-end gap-10px">
      <a-button @click="visible = false">关闭</a-button>
      <a-button type="primary" @click="saveKeyValue">保存</a-button>
    </div>
  </a-modal>
</template>
