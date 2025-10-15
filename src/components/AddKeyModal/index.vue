<script setup lang="ts">
import type { KeyType } from "@/types/redis.ts";
import { message } from "ant-design-vue";

const visible = ref(false);
const loading = ref(false);

const formData = ref({
	key: "",
	value: "",
	type: "string" as KeyType,
	ttl: -1,
});

let successCallback: ((data: typeof formData.value) => Promise<void>) | null =
	null;

// 重置表单状态
const resetForm = () => {
	formData.value = {
		key: "",
		value: "",
		type: "string",
		ttl: -1,
	};
	loading.value = false;
};

// 当对话框关闭时重置表单
watch(visible, (val) => {
	if (!val) resetForm();
});

const use = (options: {
	onSuccess: (data: typeof formData.value) => Promise<void>;
}) => {
	successCallback = options.onSuccess;
	visible.value = true;
};

const handleSubmit = async () => {
	if (!formData.value.key.trim()) {
		return;
	}

	if (!successCallback) return;

	try {
		loading.value = true;
		await successCallback({ ...formData.value });
		visible.value = false;
	} catch (error) {
		message.error("添加键失败");
	} finally {
		loading.value = false;
	}
};

defineExpose({ use });
</script>

<template>
  <a-modal
    v-model:open="visible"
    title="添加新键"
    :footer="null"
    width="600px"
    :centered="true"
    class="byte-modal"
    :maskClosable="false"
    :keyboard="false"
    :destroyOnClose="true"
  >
    <a-form layout="vertical">
      <a-form-item label="键名" required>
        <a-input
          v-model:value="formData.key"
          placeholder="输入键名"
          :disabled="loading"
        />
      </a-form-item>

      <a-form-item label="类型" required>
        <a-select
          v-model:value="formData.type"
          :disabled="loading"
        >
          <a-select-option value="string">字符串</a-select-option>
          <a-select-option value="hash">哈希</a-select-option>
          <a-select-option value="list">列表</a-select-option>
          <a-select-option value="set">集合</a-select-option>
          <a-select-option value="zset">有序集合</a-select-option>
        </a-select>
      </a-form-item>

      <a-form-item label="值" required>
        <a-textarea
          v-model:value="formData.value"
          :rows="4"
          placeholder="输入键值"
          :disabled="loading"
        />
      </a-form-item>

      <a-form-item>
        <template #label>
          <span>过期时间 (秒)</span>
          <a-tag class="ml-16px" color="blue" :bordered="false">-1 表示永不过期</a-tag>
        </template>
        <a-input-number
          class="w-200px"
          v-model:value="formData.ttl"
          :min="-1"
          :step="1"
          :disabled="loading"
        />
      </a-form-item>

      <div class="flex justify-end gap-10px">
        <a-button
          @click="visible = false"
          :disabled="loading"
        >
          取消
        </a-button>
        <a-button
          type="primary"
          @click="handleSubmit"
          :loading="loading"
        >
          添加
        </a-button>
      </div>
    </a-form>
  </a-modal>
</template>
