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

    <a-modal v-model:open="zsetItemModalVisible" :title="zsetItemModalTitle" @ok="handleZSetItemOperation" @cancel="closeZSetItemModal">
      <a-form layout="vertical">
        <a-form-item label="分数 (Score)" :help="'用于排序，分数越小越靠前。可以是任意浮点数（包括负数）'">
          <a-input-number v-model:value="zsetItem.score" :precision="2" style="width: 100%" />
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
import {
	EditOutlined,
	DeleteOutlined,
	PlusOutlined,
} from "@ant-design/icons-vue";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";
import { addZSetItem, deleteZSetItem as deleteZSetItemApi } from "@/api";

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
		if (!connectionStore?.activeConnection?.id) {
			message.error("未选择连接");
			return;
		}

		if (zsetItem.originalValue) {
			// 先删除旧值，再添加新值
			const deleteRes = await deleteZSetItemApi(
				connectionStore.activeConnection.id,
				props.keyData.key,
				zsetItem.originalValue,
			);
			if (!deleteRes.success) {
				message.error(deleteRes.message || "删除旧值失败");
				return;
			}

			const addRes = await addZSetItem(
				connectionStore.activeConnection.id,
				props.keyData.key,
				zsetItem.score,
				zsetItem.value,
			);
			if (addRes.success) {
				const index = props.keyData.value.findIndex((item: any) => item.value === zsetItem.originalValue);
				if (index !== -1) {
					props.keyData.value[index] = { score: zsetItem.score, value: zsetItem.value };
				}
				message.success(addRes.message || "元素已更新");
				zsetItemModalVisible.value = false;
			} else {
				message.error(addRes.message || "添加新值失败");
			}
		} else {
			const res = await addZSetItem(
				connectionStore.activeConnection.id,
				props.keyData.key,
				zsetItem.score,
				zsetItem.value,
			);

			if (res.success) {
				props.keyData.value.push({ score: zsetItem.score, value: zsetItem.value });
				message.success(res.message || "元素已添加");
				zsetItemModalVisible.value = false;
			} else {
				message.error(res.message || "添加失败");
			}
		}
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
				if (!connectionStore?.activeConnection?.id) {
					message.error("未选择连接");
					return;
				}

				const res = await deleteZSetItemApi(
					connectionStore.activeConnection.id,
					props.keyData.key,
					value,
				);

				if (res.success) {
					props.keyData.value = props.keyData.value.filter((item: any) => item.value !== value);
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


