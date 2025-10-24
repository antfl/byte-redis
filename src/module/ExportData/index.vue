<template>
  <a-modal
    v-model:visible="visible"
    title="导出数据"
    width="600px"
    :footer="null"
  >
    <a-form layout="vertical">
      <a-form-item label="导出范围">
        <a-radio-group v-model:value="exportScope">
          <a-radio value="current">当前键</a-radio>
          <a-radio value="all">所有键</a-radio>
          <a-radio value="pattern">匹配模式</a-radio>
        </a-radio-group>
      </a-form-item>

      <a-form-item v-if="exportScope === 'pattern'" label="键名模式">
        <a-input v-model:value="pattern" placeholder="例如: user:*" />
      </a-form-item>

      <a-form-item>
        <a-button type="primary" @click="handleExport" :loading="loading">
          <template #icon><export-outlined /></template>
          导出数据
        </a-button>
      </a-form-item>

      <a-form-item v-if="exportData">
        <a-textarea
          v-model:value="exportData"
          :rows="10"
          readonly
          placeholder="导出数据将显示在这里"
        />
        <div class="mt-10px">
          <a-button @click="copyToClipboard">
            <template #icon><copy-outlined /></template>
            复制到剪贴板
          </a-button>
          <a-button class="ml-10px" @click="downloadFile">
            <template #icon><download-outlined /></template>
            下载文件
          </a-button>
        </div>
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { message } from "ant-design-vue";
import { invoke } from "@tauri-apps/api/core";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";
import {
	ExportOutlined,
	CopyOutlined,
	DownloadOutlined,
} from "@ant-design/icons-vue";
import { computed } from "vue";

const visible = ref(false);
const exportScope = ref("current");
const pattern = ref("");
const exportData = ref("");
const loading = ref(false);

const connectionStore = useConnectionStore();

const currentKey = computed(() => connectionStore.currentKey);

const use = () => {
	visible.value = true;
};

const handleExport = async () => {
	if (!connectionStore.activeConnection?.id) {
		message.error("请先选择连接");
		return;
	}

	loading.value = true;

	try {
		if (exportScope.value === "current") {
			if (!currentKey.value) {
				throw new Error("当前没有选中的键");
			}
			const data = await invoke("export_key", {
				connectionId: connectionStore.activeConnection.id,
				key: currentKey.value,
			});
			exportData.value = JSON.stringify(data, null, 2);
		} else if (exportScope.value === "all") {
			const data = await invoke("export_keys", {
				connectionId: connectionStore.activeConnection.id,
				pattern: "*",
			});
			exportData.value = JSON.stringify(data, null, 2);
		} else if (exportScope.value === "pattern") {
			if (!pattern.value.trim()) {
				throw new Error("请输入键名模式");
			}
			const data = await invoke("export_keys", {
				connectionId: connectionStore.activeConnection.id,
				pattern: pattern.value,
			});
			exportData.value = JSON.stringify(data, null, 2);
		}
		message.success("导出成功");
	} catch (error) {
		message.error(`导出失败: ${error}`);
	} finally {
		loading.value = false;
	}
};

const copyToClipboard = async () => {
	try {
		await navigator.clipboard.writeText(exportData.value);
		message.success("已复制到剪贴板");
	} catch (error) {
		message.error("复制失败");
	}
};

const downloadFile = () => {
	const blob = new Blob([exportData.value], { type: "application/json" });
	const url = URL.createObjectURL(blob);
	const a = document.createElement("a");
	a.href = url;
	a.download = `redis-export-${new Date().toISOString().slice(0, 10)}.json`;
	document.body.appendChild(a);
	a.click();
	document.body.removeChild(a);
	URL.revokeObjectURL(url);
};

defineExpose({ use });
</script>
