<script setup lang="ts">
import { computed } from "vue";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";

const connectionStore = useConnectionStore();

const activeConnection = computed(() => connectionStore.activeConnection);
const isConnected = computed(() => Boolean(activeConnection.value));

const status = computed<{
	text: string;
	badge: "success" | "default";
}>(() => ({
	text: isConnected.value ? "已连接" : "未连接",
	badge: isConnected.value ? "success" : "default",
}));

const connectionName = computed(() => activeConnection.value?.name || "未选择连接");

const currentDb = computed(() => {
	if (!isConnected.value) return "--";
	const index = connectionStore.currentDbIndex;
	return `DB${index.toString().padStart(2, "0")}`;
});

const keyCount = computed(() => connectionStore.currentKeyCount);
const currentKey = computed(() => connectionStore.currentKey || "未选择 Key");

const infoChips = computed(() => [
	{
		key: "connection",
		value: connectionName.value,
		tooltip: "连接名称",
		className: "chip-strong",
	},
	{
		key: "db",
		value: currentDb.value,
		tooltip: "当前数据库",
	},
	{
		key: "count",
		value: `${keyCount.value}`,
		tooltip: "Key 数量",
	},
	{
		key: "currentKey",
		value: currentKey.value,
		tooltip: "当前 Key",
		className: "ellipsis",
	},
]);
</script>

<template>
	<div class="connection-status-info">
		<div class="status-chip">
			<a-badge :status="status.badge" />
			<span class="status-text">{{ status.text }}</span>
		</div>

		<div class="chips">
			<a-tooltip
				v-for="chip in infoChips"
				:key="chip.key"
				:title="chip.tooltip"
			>
				<span class="chip" :class="chip.className">
					{{ chip.value }}
				</span>
			</a-tooltip>
		</div>
	</div>
</template>

<style scoped lang="less">
.connection-status-info {
	display: flex;
	align-items: center;
	gap: 20px;
	min-width: 0;
	font-size: 13px;
	color: var(--color-text-base);
}

.status-chip {
	display: flex;
	align-items: center;
	color: var(--color-text-base);
	white-space: nowrap;
}

.status-text {
	font-weight: 500;
}

.chips {
	display: flex;
	align-items: center;
	gap: 14px;
	min-width: 0;
  cursor: default;
}

.chip {
	position: relative;
	display: inline-flex;
	align-items: center;
	max-width: 220px;
	padding-right: 4px;
	color: var(--color-text-base);
	white-space: nowrap;

	&::before {
		content: "•";
		margin-right: 8px;
		color: var(--color-text-tertiary);
	}

	&.chip-strong {
		font-weight: 600;
	}

	&.ellipsis {
		overflow: hidden;
		text-overflow: ellipsis;
	}
}
</style>

