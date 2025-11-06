<template>
  <div class="list-table">
    <a-table :dataSource="rows" :columns="listColumns" :pagination="false" size="small">
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

    <a-modal v-model:visible="listItemModalVisible" :title="listItemModalTitle" @ok="handleListItemOperation" @cancel="closeListItemModal">
      <a-form layout="vertical">
        <a-form-item label="元素值">
          <a-input v-model:value="listItem.value" />
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

interface ListItem {
	index: number;
	value: string;
}

const props = defineProps<{ keyData: RedisKey }>();
const connectionStore = useConnectionStore();

const rows = computed<ListItem[]>(() =>
	Array.isArray(props.keyData.value)
		? props.keyData.value.map((v: any, i: number) => ({ index: i, value: v }))
		: [],
);

const listColumns = [
	{ title: "索引", dataIndex: "index", key: "index", width: "80px" },
	{ title: "值", dataIndex: "value", key: "value" },
	{ title: "操作", dataIndex: "action", key: "action", width: "100px" },
];

const listItemModalVisible = ref(false);
const listItemModalTitle = ref("添加元素");
const listItem = reactive({ value: "", index: -1 });

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
				key: props.keyData.key,
				index: listItem.index,
				value: listItem.value,
			});

			props.keyData.value[listItem.index] = listItem.value;
			message.success("元素已更新");
		} else {
			await invoke("append_list_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: props.keyData.key,
				value: listItem.value,
			});

			props.keyData.value.push(listItem.value);
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
					key: props.keyData.key,
					value: record.value,
					count: 1,
				});

				props.keyData.value.splice(record.index, 1);
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
</script>

<style scoped>
.list-table {
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


