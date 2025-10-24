<template>
  <a-modal
    v-model:visible="visible"
    title="导入数据"
    width="600px"
    :footer="null"
  >
    <a-form layout="vertical">
      <a-form-item label="导入方式">
        <a-radio-group v-model:value="importMethod">
          <a-radio value="file">从文件导入</a-radio>
          <a-radio value="text">粘贴JSON数据</a-radio>
        </a-radio-group>
      </a-form-item>

      <a-form-item v-if="importMethod === 'file'">
        <a-upload
          :file-list="fileList"
          :before-upload="beforeUpload"
          accept=".json"
          :show-upload-list="false"
        >
          <a-button>
            <template #icon><upload-outlined /></template>
            选择文件
          </a-button>
        </a-upload>
        <div v-if="fileName" class="mt-10px">
          已选择: {{ fileName }}
        </div>
      </a-form-item>

      <a-form-item v-if="importMethod === 'text'">
        <a-textarea
          v-model:value="jsonData"
          :rows="6"
          placeholder="粘贴JSON格式的导出数据"
        />
      </a-form-item>

      <a-form-item label="冲突处理">
        <a-radio-group v-model:value="overwrite">
          <a-radio :value="true">覆盖已存在的键</a-radio>
          <a-radio :value="false">跳过已存在的键</a-radio>
        </a-radio-group>
      </a-form-item>

      <a-form-item>
        <a-button
          type="primary"
          @click="handleImport"
          :loading="loading"
          :disabled="!importData"
        >
          <template #icon><import-outlined /></template>
          导入数据
        </a-button>
      </a-form-item>

      <a-form-item v-if="importResult">
        <a-alert :message="importResult" :type="importSuccess ? 'success' : 'error'" />
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { message } from "ant-design-vue";
import { invoke } from "@tauri-apps/api/core";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";
import { ImportOutlined, UploadOutlined } from "@ant-design/icons-vue";

const visible = ref(false);
const importMethod = ref("file");
const fileList = ref([]);
const fileName = ref("");
const jsonData = ref("");
const overwrite = ref(true);
const importData = ref(null);
const importResult = ref("");
const importSuccess = ref(false);
const loading = ref(false);

const connectionStore = useConnectionStore();

const use = () => {
	visible.value = true;
	resetForm();
};

const resetForm = () => {
	fileList.value = [];
	fileName.value = "";
	jsonData.value = "";
	importData.value = null;
	importResult.value = "";
	importSuccess.value = false;
};

const beforeUpload = (file: any) => {
	const reader = new FileReader();
	reader.onload = (e: any) => {
		try {
			importData.value = JSON.parse(e.target.result);
			fileName.value = file.name;
		} catch (error) {
			message.error("文件解析失败，请确保是有效的JSON格式");
		}
	};
	reader.readAsText(file);
	return false; // 阻止自动上传
};

watch(jsonData, (newVal) => {
	if (newVal.trim()) {
		try {
			importData.value = JSON.parse(newVal);
		} catch (error) {
			importData.value = null;
		}
	} else {
		importData.value = null;
	}
});

const handleImport = async () => {
	if (!connectionStore.activeConnection?.id) {
		message.error("请先选择连接");
		return;
	}

	if (!importData.value) {
		message.error("请提供导入数据");
		return;
	}

	loading.value = true;
	importResult.value = "";

	try {
		let result;
		if (Array.isArray(importData.value)) {
			// 导入多个键
			result = await invoke<{ message: string; success: boolean }>(
				"import_keys",
				{
					connectionId: connectionStore.activeConnection.id,
					keys: importData.value,
					overwrite: overwrite.value,
				},
			);
		} else {
			// 导入单个键
			result = await invoke<{ message: string; success: boolean }>(
				"import_key",
				{
					connectionId: connectionStore.activeConnection.id,
					keyDetail: importData.value,
					overwrite: overwrite.value,
				},
			);
		}

		importResult.value = result.message;
		importSuccess.value = result.success;

		if (result.success) {
			message.success("导入成功");
		} else {
			message.error("导入过程中出现错误");
		}
	} catch (error) {
		importResult.value = `导入失败: ${error}`;
		importSuccess.value = false;
		message.error(`导入失败: ${error}`);
	} finally {
		loading.value = false;
	}
};

defineExpose({ use });
</script>
