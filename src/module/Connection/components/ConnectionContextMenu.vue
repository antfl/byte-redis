<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
	visible: boolean;
	position: { left: number; top: number };
}>();

const emit = defineEmits<{
	(e: "reconnect"): void;
	(e: "disconnect"): void;
	(e: "edit"): void;
	(e: "delete"): void;
}>();

const menuStyle = computed(() => ({
	left: `${props.position.left}px`,
	top: `${props.position.top}px`,
}));

type ActionType = "reconnect" | "disconnect" | "edit" | "delete";

const handleAction = (action: ActionType) => {
	switch (action) {
		case "reconnect":
			emit("reconnect");
			break;
		case "disconnect":
			emit("disconnect");
			break;
		case "edit":
			emit("edit");
			break;
		case "delete":
			emit("delete");
			break;
	}
};
</script>

<template>
	<Teleport to="body">
		<div
			v-if="visible"
			class="connection-context-menu"
			:style="menuStyle"
			@click.stop
		>
			<a-menu :selectable="false">
				<a-menu-item key="reconnect" @click="handleAction('reconnect')">
					重新
				</a-menu-item>
				<a-menu-item key="disconnect" @click="handleAction('disconnect')">
					断开
				</a-menu-item>
				<a-menu-item key="edit" @click="handleAction('edit')">
					编辑
				</a-menu-item>
				<a-menu-item key="delete" @click="handleAction('delete')">
					删除
				</a-menu-item>
			</a-menu>
		</div>
	</Teleport>
</template>

<style scoped lang="less">
.connection-context-menu {
	position: fixed;
	z-index: 3000;
	background-color: #fff;
	box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
	border-radius: 6px;
	overflow: hidden;
  :deep(.ant-menu-item) {
    height: 32px;
    line-height: 32px;
  }
}
</style>

