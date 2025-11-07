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

    <a-modal v-model:open="setItemModalVisible" :title="setItemModalTitle" @ok="handleSetItemOperation" @cancel="closeSetItemModal">
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
import {
	EditOutlined,
	DeleteOutlined,
	PlusOutlined,
} from "@ant-design/icons-vue";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";
import { addSetItem, deleteSetItem as deleteSetItemApi } from "@/api";

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
		if (!connectionStore?.activeConnection?.id) {
			message.error("未选择连接");
			return;
		}

		if (setItem.originalValue) {
			// 先删除旧值，再添加新值
			const deleteRes = await deleteSetItemApi(
				connectionStore.activeConnection.id,
				props.keyData.key,
				setItem.originalValue,
			);
			if (!deleteRes.success) {
				message.error(deleteRes.message || "删除旧值失败");
				return;
			}

			const addRes = await addSetItem(
				connectionStore.activeConnection.id,
				props.keyData.key,
				setItem.value,
			);
			if (addRes.success) {
				const index = props.keyData.value.findIndex((v: string) => v === setItem.originalValue);
				if (index !== -1) {
					props.keyData.value[index] = setItem.value;
				}
				message.success(addRes.message || "元素已更新");
				setItemModalVisible.value = false;
			} else {
				message.error(addRes.message || "添加新值失败");
			}
		} else {
			const res = await addSetItem(
				connectionStore.activeConnection.id,
				props.keyData.key,
				setItem.value,
			);

			if (res.success) {
				props.keyData.value.push(setItem.value);
				message.success(res.message || "元素已添加");
				setItemModalVisible.value = false;
			} else {
				message.error(res.message || "添加失败");
			}
		}
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
				if (!connectionStore?.activeConnection?.id) {
					message.error("未选择连接");
					return;
				}

				const res = await deleteSetItemApi(
					connectionStore.activeConnection.id,
					props.keyData.key,
					value,
				);

				if (res.success) {
					props.keyData.value = props.keyData.value.filter((v: string) => v !== value);
					message.success(res.message || "元素已删除");
				} else {
					message.error(res.message || "删除失败");
				}
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


