<template>
  <div class="w-full p-10px redis-key-detail-container bg-[var(--color-bg-container)]">
    <div class="header">
      <div>
        <div class="flex items-center">
          <a-tag class="font-700" :color="getTypeColor(keyData.type)" :bordered="false">
            {{ keyData.type.toUpperCase() }}
          </a-tag>
          <a-tag @click="copyKey" class="cursor-pointer border-dashed">
            <SwitcherOutlined />
            <span>{{ keyData.key }}</span>
          </a-tag>
          <div class="flex gap-5px">
            <IconButton  @click="showEditModal">
              <edit-outlined />
            </IconButton>
            <IconButton  @click="showDeleteConfirm">
              <delete-outlined />
            </IconButton>
            <IconButton @click="showTTLModal">
              <field-time-outlined />
            </IconButton>
          </div>
        </div>
        <div class="mt-10px flex  gap-5px">
          <div>
            <span class="color-#666">TTL：</span>
            <span>{{ keyData.ttl }} 秒</span>
          </div>
          <div>
            <span class="color-#666">大小：</span>
            <span>{{ formatSize(keyData.size) }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="content-section">
      <StringEditor v-if="keyData.type === 'string'" :keyData="keyData" />

      <HashEditor v-if="keyData.type === 'hash'" :keyData="keyData" />

      <ListEditor v-if="keyData.type === 'list'" :keyData="keyData" />

      <SetEditor v-if="keyData.type === 'set'" :keyData="keyData" />

      <ZSetEditor v-if="keyData.type === 'zset'" :keyData="keyData" />
    </div>

    <div class="footer-actions">
      <a-button type="primary" @click="saveChanges">保存修改</a-button>
      <a-button @click="refreshData">刷新数据</a-button>
    </div>

    <a-modal
      v-model:open="editModalVisible"
      title="修改Key名称"
      @ok="handleRenameKey"
      @cancel="editModalVisible = false"
    >
      <a-form layout="vertical">
        <a-form-item label="新Key名称">
          <a-input v-model:value="newKeyName" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal
      v-model:open="ttlModalVisible"
      title="修改过期时间"
      @ok="handleUpdateTTL"
      @cancel="ttlModalVisible = false"
    >
      <a-form layout="vertical">
        <a-form-item label="TTL (秒)">
          <a-input-number v-model:value="newTTL" :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item>
          <a-checkbox v-model:checked="setPersistent">设置为永久有效</a-checkbox>
        </a-form-item>
      </a-form>
    </a-modal>


  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue';
import { message, Modal } from "ant-design-vue";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";
import { renameKey, setKeyTtl, deleteKey, setKey, getKeyDetail } from "@/api";
import { useClipboard } from "@vueuse/core";
import IconButton from "@/components/IconButton/index.vue";
import StringEditor from "./components/StringEditor.vue";
import HashEditor from "./components/HashEditor.vue";
import ListEditor from "./components/ListEditor.vue";
import SetEditor from "./components/SetEditor.vue";
import ZSetEditor from "./components/ZSetEditor.vue";
import { EditOutlined, DeleteOutlined, FieldTimeOutlined, SwitcherOutlined } from "@ant-design/icons-vue";

interface RedisKey {
	key: string;
	type: string;
	ttl: number;
	size: number;
	value: any;
}


const connectionStore = useConnectionStore();
const keyData = reactive<RedisKey>({
	key: "",
	type: "",
	ttl: 0,
	size: 0,
	value: "",
});

const editModalVisible = ref(false);
const ttlModalVisible = ref(false);
const newKeyName = ref("");
const newTTL = ref(0);
const setPersistent = ref(false);

const loadKeyDetail = async () => {
	if (!connectionStore.activeConnection?.id || !connectionStore.currentKey)
		return;

	try {
		const res = await getKeyDetail(
			connectionStore.activeConnection.id,
			connectionStore.currentKey,
		);

		if (res.success && res.data) {
			Object.assign(keyData, res.data);
		} else {
			message.error(res.message || "获取键详情失败");
		}
	} catch (error) {
		message.error(`获取键详情失败: ${error}`);
	}
};

watch(() => connectionStore.currentKey, loadKeyDetail, { immediate: true });

const getTypeColor = (type: string) => {
	const colors: Record<string, string> = {
		string: "blue",
		hash: "green",
		list: "orange",
		set: "purple",
		zset: "red",
	};
	return colors[type] || "gray";
};

const copyKey = () => {
	const { copy } = useClipboard();
	copy(keyData.key);
	message.success("已复制");
};

const formatSize = (bytes: number) => {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
	return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
};

const showEditModal = () => {
	newKeyName.value = keyData.key;
	editModalVisible.value = true;
};

const showTTLModal = () => {
	newTTL.value = keyData.ttl;
	setPersistent.value = keyData.ttl === -1;
	ttlModalVisible.value = true;
};

const showDeleteConfirm = () => {
	Modal.confirm({
		title: "确认删除Key",
		content: `确定要删除Key "${keyData.key}" 吗？此操作不可恢复。`,
		okText: "删除",
		okType: "danger",
		cancelText: "取消",
		async onOk() {
			await handleDeleteKey();
		},
	});
};

const handleRenameKey = async () => {
	if (!newKeyName.value.trim()) {
		message.error("Key名称不能为空");
		return;
	}

	try {
		if (!connectionStore?.activeConnection?.id) {
			message.error("未选择连接");
			return;
		}
		
		const res = await renameKey(
			connectionStore.activeConnection.id,
			keyData.key,
			newKeyName.value,
		);

		if (res.success) {
			keyData.key = newKeyName.value;
			connectionStore.currentKey = newKeyName.value;
			// 刷新左侧 key 列表
			connectionStore.refreshKeyList();
			message.success(res.message || "Key名称已更新");
			editModalVisible.value = false;
		} else {
			message.error(res.message || "重命名失败");
		}
	} catch (error) {
		message.error(`重命名失败: ${error}`);
	}
};

const handleUpdateTTL = async () => {
	const ttlValue = setPersistent.value ? -1 : newTTL.value;

	try {
		if (!connectionStore?.activeConnection?.id) {
			message.error("未选择连接");
			return;
		}
		
		const res = await setKeyTtl(
			connectionStore.activeConnection.id,
			keyData.key,
			ttlValue,
		);

		if (res.success) {
			keyData.ttl = ttlValue;
			message.success(res.message || "TTL已更新");
			ttlModalVisible.value = false;
		} else {
			message.error(res.message || "更新TTL失败");
		}
	} catch (error) {
		message.error(`更新TTL失败: ${error}`);
	}
};

const handleDeleteKey = async () => {
	try {
		if (!connectionStore?.activeConnection?.id) {
			message.error("未选择连接");
			return;
		}
		
		const res = await deleteKey(
			connectionStore.activeConnection.id,
			keyData.key,
		);

		if (res.success) {
			message.success(res.message || `Key "${keyData.key}" 已删除`);
			connectionStore.currentKey = null;
			// 刷新左侧 key 列表
			connectionStore.refreshKeyList();
		} else {
			message.error(res.message || "删除失败");
		}
	} catch (error) {
		message.error(`删除失败: ${error}`);
	}
};

const saveChanges = async () => {
	if (!connectionStore.activeConnection?.id) {
		message.error("未选择连接");
		return;
	}

	try {
		if (keyData.type === "string") {
			const res = await setKey(
				connectionStore.activeConnection.id,
				keyData.key,
				"string",
				keyData.value,
				keyData.ttl > 0 ? keyData.ttl : 0,
			);
			if (res.success) {
				message.success(res.message || "保存成功");
			} else {
				message.error(res.message || "保存失败");
			}
		} else if (keyData.type === "hash") {
			message.info("哈希类型修改已通过字段操作完成");
		} else if (keyData.type === "list") {
			message.info("列表类型修改已通过元素操作完成");
		} else if (keyData.type === "set") {
			message.info("集合类型修改已通过元素操作完成");
		} else if (keyData.type === "zset") {
			message.info("有序集合类型修改已通过元素操作完成");
		}

		message.success("所有修改已保存");
	} catch (error) {
		message.error(`保存失败: ${error}`);
	}
};

const refreshData = async () => {
	await loadKeyDetail();
	message.success("数据已刷新");
};
</script>

<style scoped>
.header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid #f0f0f0;
}
.content-section {
  flex: 1;
  overflow-y: auto;
  margin-bottom: 24px;
}

.footer-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 16px;
  border-top: 1px solid #f0f0f0;
}
</style>

