<template>
  <div class="set-table">
    <a-table :dataSource="setItems" :columns="setColumns" :pagination="false" size="small">
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

    <a-modal v-model:visible="setItemModalVisible" :title="setItemModalTitle" @ok="handleSetItemOperation" @cancel="closeSetItemModal">
      <a-form layout="vertical">
        <a-form-item label="元素值">
          <a-input v-model:value="setItem.value" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import { message, Modal } from "ant-design-vue";
import { invoke } from "@tauri-apps/api/core";
import {
	EditOutlined,
	DeleteOutlined,
	PlusOutlined,
} from "@ant-design/icons-vue";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";

interface RedisKey {
	key: string;
	type: string;
	ttl: number;
	size: number;
	value: any;
}

interface SetItem {
	value: string;
}

const props = defineProps<{ keyData: RedisKey }>();
const connectionStore = useConnectionStore();

const setItems = computed<SetItem[]>(() =>
	props.keyData.type !== "set" || !Array.isArray(props.keyData.value)
		? []
		: props.keyData.value.map((v: string) => ({ value: v })),
);

const setColumns = [
	{ title: "值", dataIndex: "value", key: "value" },
	{ title: "操作", dataIndex: "action", key: "action", width: "100px" },
];

const setItemModalVisible = ref(false);
const setItemModalTitle = ref("添加元素");
const setItem = reactive({ value: "", originalValue: "" });

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
				key: props.keyData.key,
				value: setItem.originalValue,
			});

			await invoke("add_set_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: props.keyData.key,
				value: setItem.value,
			});

			const index = props.keyData.value.findIndex((v: string) => v === setItem.originalValue);
			if (index !== -1) {
				props.keyData.value[index] = setItem.value;
			}
			message.success("元素已更新");
		} else {
			await invoke("add_set_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: props.keyData.key,
				value: setItem.value,
			});

			props.keyData.value.push(setItem.value);
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
					key: props.keyData.key,
					value,
				});

				props.keyData.value = props.keyData.value.filter((v: string) => v !== value);
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
</script>

<style scoped>
.set-table {
  border: 1px solid #f0f0f0;
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 20px;
}
.add-item {
  margin-top: 12px;
  padding: 0 16px 16px;
}
</style>


