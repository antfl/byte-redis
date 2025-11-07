<template>
  <div class="hash-table">
    <a-table :dataSource="rows" :columns="hashColumns" :pagination="false" size="small">
      <template #bodyCell="{ column, record }">
        <template v-if="column.dataIndex === 'action'">
          <a-space>
            <a-button size="small" @click="editHashField(record)">
              <edit-outlined />
            </a-button>
            <a-button size="small" @click="deleteHashField(record.field)">
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

    <a-modal v-model:open="hashFieldModalVisible" :title="hashFieldModalTitle" @ok="handleHashFieldOperation" @cancel="closeHashFieldModal">
      <a-form layout="vertical">
        <a-form-item label="字段名">
          <a-input v-model:value="hashField.field" :disabled="isEditingHashField" />
        </a-form-item>
        <a-form-item label="字段值">
          <a-input v-model:value="hashField.value" />
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
import { updateHashField, deleteHashField as deleteHashFieldApi } from "@/api";

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

const props = defineProps<{ keyData: RedisKey }>();
const connectionStore = useConnectionStore();

const rows = computed<HashItem[]>(() => (Array.isArray(props.keyData.value) ? props.keyData.value : []));

const hashColumns = [
	{ title: "字段", dataIndex: "field", key: "field" },
	{ title: "值", dataIndex: "value", key: "value" },
	{ title: "操作", dataIndex: "action", key: "action", width: "100px" },
];

const hashFieldModalVisible = ref(false);
const isEditingHashField = ref(false);
const hashFieldModalTitle = ref("添加字段");
const hashField = reactive({ field: "", value: "" });

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
		if (!connectionStore?.activeConnection?.id) {
			message.error("未选择连接");
			return;
		}

		const res = await updateHashField(
			connectionStore.activeConnection.id,
			props.keyData.key,
			hashField.field,
			hashField.value,
		);

		if (res.success) {
			const index = props.keyData.value.findIndex((item: HashItem) => item.field === hashField.field);
			if (index !== -1) {
				// 字段已存在，更新值
				props.keyData.value[index].value = hashField.value;
			} else {
				// 字段不存在，添加新字段
				props.keyData.value.push({ field: hashField.field, value: hashField.value });
			}
			message.success(res.message || (isEditingHashField.value ? "字段已更新" : "字段已添加"));
			hashFieldModalVisible.value = false;
		} else {
			message.error(res.message || "操作失败");
		}
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
				if (!connectionStore?.activeConnection?.id) {
					message.error("未选择连接");
					return;
				}

				const res = await deleteHashFieldApi(
					connectionStore.activeConnection.id,
					props.keyData.key,
					field,
				);

				if (res.success) {
					props.keyData.value = props.keyData.value.filter((item: HashItem) => item.field !== field);
					message.success(res.message || "字段已删除");
				} else {
					message.error(res.message || "删除失败");
				}
			} catch (error) {
				message.error(`删除失败: ${error}`);
			}
		},
	});
};

const closeHashFieldModal = () => {
	hashFieldModalVisible.value = false;
};
</script>

<style scoped>
.hash-table {
  border: 1px solid #f0f0f0;
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 20px;
}
.add-field {
  margin-top: 12px;
  padding: 0 16px 16px;
}
</style>


