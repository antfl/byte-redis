<template>
  <div class="zset-table">
    <a-table :dataSource="zsetItems" :columns="zsetColumns" :pagination="false" size="small">
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

    <a-modal v-model:visible="zsetItemModalVisible" :title="zsetItemModalTitle" @ok="handleZSetItemOperation" @cancel="closeZSetItemModal">
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

interface ZSetItem {
	score: number;
	value: string;
}

const props = defineProps<{ keyData: RedisKey }>();
const connectionStore = useConnectionStore();

const zsetItems = computed<ZSetItem[]>(() =>
	props.keyData.type !== "zset" || !Array.isArray(props.keyData.value)
		? []
		: props.keyData.value.map((item: any) => ({ score: item.score, value: item.value })),
);

const zsetColumns = [
	{ title: "分数", dataIndex: "score", key: "score", width: "100px" },
	{ title: "值", dataIndex: "value", key: "value" },
	{ title: "操作", dataIndex: "action", key: "action", width: "100px" },
];

const zsetItemModalVisible = ref(false);
const zsetItemModalTitle = ref("添加元素");
const zsetItem = reactive({ score: 0, value: "", originalValue: "" });

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
				key: props.keyData.key,
				value: zsetItem.originalValue,
			});

			await invoke("add_zset_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: props.keyData.key,
				score: zsetItem.score,
				value: zsetItem.value,
			});

			const index = props.keyData.value.findIndex((item: any) => item.value === zsetItem.originalValue);
			if (index !== -1) {
				props.keyData.value[index] = { score: zsetItem.score, value: zsetItem.value };
			}
			message.success("元素已更新");
		} else {
			await invoke("add_zset_item", {
				connectionId: connectionStore?.activeConnection?.id,
				key: props.keyData.key,
				score: zsetItem.score,
				value: zsetItem.value,
			});

			props.keyData.value.push({ score: zsetItem.score, value: zsetItem.value });
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
					key: props.keyData.key,
					value,
				});

				props.keyData.value = props.keyData.value.filter((item: any) => item.value !== value);
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
.zset-table {
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


