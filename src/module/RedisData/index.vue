<template>
  <div class="w-full p-10px redis-key-detail-container bg-#fff">
    <div class="header">
      <div>
        <div class="flex items-center">
          <a-tag class="font-700" :color="getTypeColor(keyData.type)" :bordered="false">
            {{ keyData.type.toUpperCase() }}
          </a-tag>
          <a-tag @click="copyKey" class="cursor-pointer border-dashed" color="geekblue">
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
      <div v-if="keyData.type === 'string'" class="value-editor">
        <a-textarea
          v-model:value="keyData.value"
          placeholder="值内容"
          :auto-size="{ minRows: 4, maxRows: 10 }"
        />
      </div>

      <div v-if="keyData.type === 'hash'" class="hash-table">
        <a-table
          :dataSource="keyData.value"
          :columns="hashColumns"
          :pagination="false"
          size="small"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.dataIndex === 'action'">
              <a-space>
                <a-button  size="small" @click="editHashField(record)">
                  <edit-outlined />
                </a-button>
                <a-button   size="small" @click="deleteHashField(record.field)">
                  <delete-outlined />
                </a-button>
              </a-space>
            </template>
          </template>
        </a-table>
        <div class="add-field">
          <a-button type="dashed" @click="showAddHashFieldModal">
            <plus-outlined />
            添加字段
          </a-button>
        </div>
      </div>

      <div v-if="keyData.type === 'list'" class="list-table">
        <a-table
          :dataSource="keyData.value.map((v:any, i: number) => ({ index: i, value: v }))"
          :columns="listColumns"
          :pagination="false"
          size="small"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.dataIndex === 'action'">
              <a-space>
                <a-button type="link" size="small" @click="editListItem(record)">
                  <edit-outlined />
                </a-button>
                <a-button type="link" danger size="small" @click="deleteListItem(record)">
                  <delete-outlined />
                </a-button>
              </a-space>
            </template>
          </template>
        </a-table>
        <div class="add-item">
          <a-button type="dashed" @click="showAddListItemModal">
            <plus-outlined />
            添加元素
          </a-button>
        </div>
      </div>

      <div v-if="keyData.type === 'set'" class="set-table">
        <a-table
          :dataSource="setItems"
          :columns="setColumns"
          :pagination="false"
          size="small"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.dataIndex === 'action'">
              <a-space>
                <a-button type="link" size="small" @click="editSetItem(record)">
                  <edit-outlined />
                </a-button>
                <a-button type="link" danger size="small" @click="deleteSetItem(record.value)">
                  <delete-outlined />
                </a-button>
              </a-space>
            </template>
          </template>
        </a-table>
        <div class="add-item">
          <a-button type="dashed" @click="showAddSetItemModal">
            <plus-outlined />
            添加元素
          </a-button>
        </div>
      </div>

      <div v-if="keyData.type === 'zset'" class="zset-table">
        <a-table
          :dataSource="zsetItems"
          :columns="zsetColumns"
          :pagination="false"
          size="small"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.dataIndex === 'action'">
              <a-space>
                <a-button type="link" size="small" @click="editZSetItem(record)">
                  <edit-outlined />
                </a-button>
                <a-button type="link" danger size="small" @click="deleteZSetItem(record.value)">
                  <delete-outlined />
                </a-button>
              </a-space>
            </template>
          </template>
        </a-table>
        <div class="add-item">
          <a-button type="dashed" @click="showAddZSetItemModal">
            <plus-outlined />
            添加元素
          </a-button>
        </div>
      </div>
    </div>

    <div class="footer-actions">
      <a-button type="primary" @click="saveChanges">保存修改</a-button>
      <a-button @click="refreshData">刷新数据</a-button>
    </div>

    <a-modal
      v-model:visible="editModalVisible"
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
      v-model:visible="ttlModalVisible"
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

    <a-modal
      v-model:visible="hashFieldModalVisible"
      :title="hashFieldModalTitle"
      @ok="handleHashFieldOperation"
      @cancel="closeHashFieldModal"
    >
      <a-form layout="vertical">
        <a-form-item label="字段名">
          <a-input v-model:value="hashField.field" :disabled="isEditingHashField" />
        </a-form-item>
        <a-form-item label="字段值">
          <a-input v-model:value="hashField.value" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal
      v-model:visible="listItemModalVisible"
      :title="listItemModalTitle"
      @ok="handleListItemOperation"
      @cancel="closeListItemModal"
    >
      <a-form layout="vertical">
        <a-form-item label="元素值">
          <a-input v-model:value="listItem.value" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal
      v-model:visible="setItemModalVisible"
      :title="setItemModalTitle"
      @ok="handleSetItemOperation"
      @cancel="closeSetItemModal"
    >
      <a-form layout="vertical">
        <a-form-item label="元素值">
          <a-input v-model:value="setItem.value" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal
      v-model:visible="zsetItemModalVisible"
      :title="zsetItemModalTitle"
      @ok="handleZSetItemOperation"
      @cancel="closeZSetItemModal"
    >
      <a-form layout="vertical">
        <a-form-item label="分数">
          <a-input-number v-model:value="zsetItem.score" :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item label="元素值">
          <a-input v-model:value="zsetItem.value" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch, computed } from "vue";
import { message, Modal } from "ant-design-vue";
import { invoke } from "@tauri-apps/api/core";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";
import { useClipboard } from "@vueuse/core";
import IconButton from "@/components/IconButton/index.vue";

import {
	EditOutlined,
	DeleteOutlined,
	FieldTimeOutlined,
	PlusOutlined,
} from "@ant-design/icons-vue";

interface RedisKey {
	key: string;
	type: string;
	ttl: number;
	size: number;
	value: any;
}

interface HashItem {
	field: string;
	value: string;
}

interface ListItem {
	index: number;
	value: string;
}

interface SetItem {
	value: string;
}

interface ZSetItem {
	score: number;
	value: string;
}

interface KeyDetailResponse {
	detail: RedisKey;
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
const hashFieldModalVisible = ref(false);
const listItemModalVisible = ref(false);
const setItemModalVisible = ref(false);
const zsetItemModalVisible = ref(false);
const newKeyName = ref("");
const newTTL = ref(0);
const setPersistent = ref(false);
const isEditingHashField = ref(false);
const hashFieldModalTitle = ref("添加字段");
const listItemModalTitle = ref("添加元素");
const setItemModalTitle = ref("添加元素");
const zsetItemModalTitle = ref("添加元素");

const hashField = reactive({
	field: "",
	value: "",
});

const listItem = reactive({
	value: "",
	index: -1,
});

const setItem = reactive({
	value: "",
	originalValue: "",
});

const zsetItem = reactive({
	score: 0,
	value: "",
	originalValue: "",
});

const hashColumns = [
	{ title: "字段", dataIndex: "field", key: "field" },
	{ title: "值", dataIndex: "value", key: "value" },
	{ title: "操作", dataIndex: "action", key: "action", width: "100px" },
];

const listColumns = [
	{ title: "索引", dataIndex: "index", key: "index", width: "80px" },
	{ title: "值", dataIndex: "value", key: "value" },
	{ title: "操作", dataIndex: "action", key: "action", width: "100px" },
];

const setColumns = [
	{ title: "值", dataIndex: "value", key: "value" },
	{ title: "操作", dataIndex: "action", key: "action", width: "100px" },
];

const zsetColumns = [
	{ title: "分数", dataIndex: "score", key: "score", width: "100px" },
	{ title: "值", dataIndex: "value", key: "value" },
	{ title: "操作", dataIndex: "action", key: "action", width: "100px" },
];

const setItems = computed(() => {
	if (keyData.type !== "set") return [];
	return keyData.value.map((v: string) => ({ value: v }));
});

const zsetItems = computed(() => {
	if (keyData.type !== "zset") return [];
	return keyData.value.map((item: any) => ({
		score: item.score,
		value: item.value,
	}));
});

const loadKeyDetail = async () => {
	if (!connectionStore.activeConnection?.id || !connectionStore.currentKey)
		return;

	try {
		const res = await invoke<KeyDetailResponse>("get_key_detail", {
			connectionId: connectionStore.activeConnection.id,
			key: connectionStore.currentKey,
		});

		Object.assign(keyData, res.detail);
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
		await invoke("rename_key", {
			connectionId: connectionStore?.activeConnection?.id,
			oldKey: keyData.key,
			newKey: newKeyName.value,
		});

		keyData.key = newKeyName.value;
		connectionStore.currentKey = newKeyName.value;
		message.success("Key名称已更新");
		editModalVisible.value = false;
	} catch (error) {
		message.error(`重命名失败: ${error}`);
	}
};

const handleUpdateTTL = async () => {
	const ttlValue = setPersistent.value ? -1 : newTTL.value;

	try {
		await invoke("set_key_ttl", {
			connectionId: connectionStore?.activeConnection?.id,
			key: keyData.key,
			ttl: ttlValue,
		});

		keyData.ttl = ttlValue;
		message.success("TTL已更新");
		ttlModalVisible.value = false;
	} catch (error) {
		message.error(`更新TTL失败: ${error}`);
	}
};

const handleDeleteKey = async () => {
	try {
		await invoke("delete_key", {
			connectionId: connectionStore?.activeConnection?.id,
			key: keyData.key,
		});

		message.success(`Key "${keyData.key}" 已删除`);
		connectionStore.currentKey = null;
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
			await invoke("set_key", {
				connectionId: connectionStore.activeConnection.id,
				key: keyData.key,
				value: keyData.value,
				ttl: keyData.ttl > 0 ? keyData.ttl : 0,
			});
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

const showAddHashFieldModal = () => {
	hashField.field = "";
	hashField.value = "";
	isEditingHashField.value = false;
	hashFieldModalTitle.value = "添加字段";
	hashFieldModalVisible.value = true;
};

const editHashField = (record: HashItem) => {
	hashField.field = record.field;
	hashField.value = record.value;
	isEditingHashField.value = true;
	hashFieldModalTitle.value = "修改字段";
	hashFieldModalVisible.value = true;
};

const handleHashFieldOperation = async () => {
	if (!hashField.field || !hashField.value) {
		message.error("字段名和值不能为空");
		return;
	}

	try {
		if (isEditingHashField.value) {
			await invoke("update_hash_field", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				field: hashField.field,
				value: hashField.value,
			});

			const index = keyData.value.findIndex(
				(item: HashItem) => item.field === hashField.field,
			);
			if (index !== -1) {
				keyData.value[index].value = hashField.value;
			}
			message.success("字段已更新");
		} else {
			await invoke("update_hash_field", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				field: hashField.field,
				value: hashField.value,
			});

			keyData.value.push({
				field: hashField.field,
				value: hashField.value,
			});
			message.success("字段已添加");
		}

		hashFieldModalVisible.value = false;
	} catch (error) {
		message.error(`操作失败: ${error}`);
	}
};

const deleteHashField = (field: string) => {
	Modal.confirm({
		title: "确认删除字段",
		content: `确定要删除字段 "${field}" 吗？`,
		okText: "删除",
		okType: "danger",
		cancelText: "取消",
		async onOk() {
			try {
				await invoke("delete_hash_field", {
					connectionId: connectionStore?.activeConnection?.id,
					key: keyData.key,
					field: field,
				});

				keyData.value = keyData.value.filter(
					(item: HashItem) => item.field !== field,
				);
				message.success("字段已删除");
			} catch (error) {
				message.error(`删除失败: ${error}`);
			}
		},
	});
};

const closeHashFieldModal = () => {
	hashFieldModalVisible.value = false;
};

const showAddListItemModal = () => {
	listItem.value = "";
	listItem.index = -1;
	listItemModalTitle.value = "添加元素";
	listItemModalVisible.value = true;
};

const editListItem = (record: ListItem) => {
	listItem.value = record.value;
	listItem.index = record.index;
	listItemModalTitle.value = "修改元素";
	listItemModalVisible.value = true;
};

const handleListItemOperation = async () => {
	if (!listItem.value) {
		message.error("元素值不能为空");
		return;
	}

	try {
		if (listItem.index >= 0) {
			await invoke("update_list_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				index: listItem.index,
				value: listItem.value,
			});

			keyData.value[listItem.index] = listItem.value;
			message.success("元素已更新");
		} else {
			await invoke("append_list_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				value: listItem.value,
			});

			keyData.value.push(listItem.value);
			message.success("元素已添加");
		}

		listItemModalVisible.value = false;
	} catch (error) {
		message.error(`操作失败: ${error}`);
	}
};

const deleteListItem = (record: ListItem) => {
	Modal.confirm({
		title: "确认删除元素",
		content: "确定要删除此元素吗？",
		okText: "删除",
		okType: "danger",
		cancelText: "取消",
		async onOk() {
			try {
				await invoke("delete_list_item", {
					connectionId: connectionStore?.activeConnection?.id,
					key: keyData.key,
					value: record.value,
					count: 1,
				});

				keyData.value.splice(record.index, 1);
				message.success("元素已删除");
			} catch (error) {
				message.error(`删除失败: ${error}`);
			}
		},
	});
};

const closeListItemModal = () => {
	listItemModalVisible.value = false;
};

const showAddSetItemModal = () => {
	setItem.value = "";
	setItem.originalValue = "";
	setItemModalTitle.value = "添加元素";
	setItemModalVisible.value = true;
};

const editSetItem = (record: SetItem) => {
	setItem.value = record.value;
	setItem.originalValue = record.value;
	setItemModalTitle.value = "修改元素";
	setItemModalVisible.value = true;
};

const handleSetItemOperation = async () => {
	if (!setItem.value.trim()) {
		message.error("元素值不能为空");
		return;
	}

	try {
		if (setItem.originalValue) {
			await invoke("delete_set_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				value: setItem.originalValue,
			});

			await invoke("add_set_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				value: setItem.value,
			});

			const index = keyData.value.findIndex(
				(v: string) => v === setItem.originalValue,
			);
			if (index !== -1) {
				keyData.value[index] = setItem.value;
			}
			message.success("元素已更新");
		} else {
			await invoke("add_set_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				value: setItem.value,
			});

			keyData.value.push(setItem.value);
			message.success("元素已添加");
		}

		setItemModalVisible.value = false;
	} catch (error) {
		message.error(`操作失败: ${error}`);
	}
};

const deleteSetItem = (value: string) => {
	Modal.confirm({
		title: "确认删除元素",
		content: "确定要删除此元素吗？",
		okText: "删除",
		okType: "danger",
		cancelText: "取消",
		async onOk() {
			try {
				await invoke("delete_set_item", {
					connectionId: connectionStore?.activeConnection?.id,
					key: keyData.key,
					value: value,
				});

				keyData.value = keyData.value.filter((v: string) => v !== value);
				message.success("元素已删除");
			} catch (error) {
				message.error(`删除失败: ${error}`);
			}
		},
	});
};

const closeSetItemModal = () => {
	setItemModalVisible.value = false;
};

const showAddZSetItemModal = () => {
	zsetItem.score = 0;
	zsetItem.value = "";
	zsetItem.originalValue = "";
	zsetItemModalTitle.value = "添加元素";
	zsetItemModalVisible.value = true;
};

const editZSetItem = (record: ZSetItem) => {
	zsetItem.score = record.score;
	zsetItem.value = record.value;
	zsetItem.originalValue = record.value;
	zsetItemModalTitle.value = "修改元素";
	zsetItemModalVisible.value = true;
};

const handleZSetItemOperation = async () => {
	if (!zsetItem.value.trim()) {
		message.error("元素值不能为空");
		return;
	}

	try {
		if (zsetItem.originalValue) {
			await invoke("delete_zset_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				value: zsetItem.originalValue,
			});

			await invoke("add_zset_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				score: zsetItem.score,
				value: zsetItem.value,
			});

			const index = keyData.value.findIndex(
				(item: any) => item.value === zsetItem.originalValue,
			);
			if (index !== -1) {
				keyData.value[index] = {
					score: zsetItem.score,
					value: zsetItem.value,
				};
			}
			message.success("元素已更新");
		} else {
			await invoke("add_zset_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: keyData.key,
				score: zsetItem.score,
				value: zsetItem.value,
			});

			keyData.value.push({
				score: zsetItem.score,
				value: zsetItem.value,
			});
			message.success("元素已添加");
		}

		zsetItemModalVisible.value = false;
	} catch (error) {
		message.error(`操作失败: ${error}`);
	}
};

const deleteZSetItem = (value: string) => {
	Modal.confirm({
		title: "确认删除元素",
		content: "确定要删除此元素吗？",
		okText: "删除",
		okType: "danger",
		cancelText: "取消",
		async onOk() {
			try {
				await invoke("delete_zset_item", {
					connectionId: connectionStore?.activeConnection?.id,
					key: keyData.key,
					value: value,
				});

				keyData.value = keyData.value.filter(
					(item: any) => item.value !== value,
				);
				message.success("元素已删除");
			} catch (error) {
				message.error(`删除失败: ${error}`);
			}
		},
	});
};

const closeZSetItemModal = () => {
	zsetItemModalVisible.value = false;
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

.value-editor {
  margin-bottom: 16px;
}

.hash-table, .list-table, .set-table, .zset-table {
  border: 1px solid #f0f0f0;
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 20px;
}

.add-field, .add-item {
  margin-top: 12px;
  padding: 0 16px 16px;
}

.footer-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 16px;
  border-top: 1px solid #f0f0f0;
}
</style>
