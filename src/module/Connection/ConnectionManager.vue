<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { PlusOutlined } from '@ant-design/icons-vue';
import ConnectionModal, { type ConnectionFormState } from "@/module/Connection/ConnectionModal.vue";
import downOutlined from "@/assets/svg/down-outlined.svg";
import { Connection, useConnectionStore } from "@/stores/useConnectionStore.ts";
import IconButton from "@/components/IconButton/index.vue";
import { connectRedis, disconnectRedis } from "@/api";
import { message, Modal } from "ant-design-vue";
import ConnectionContextMenu from "@/module/Connection/components/ConnectionContextMenu.vue";

const connectionStore = useConnectionStore();

const connections = computed(() => connectionStore.connections);

const onClick = (item: Connection) => {
	connectionStore.setActiveConnection(item.id);
};

const ConnectionModalRef = ref();
const newConnection = () => {
	ConnectionModalRef.value.open({
		onSuccess: (data: ConnectionFormState) => {
			connectionStore.createConnection({
				name: data.name,
				host: data.host,
				port: data.port,
				username: data.username ? data.username : null,
				password: data.password ? data.password : null,
				separator: data.separator || ":",
				db: data.db ?? 0,
			});
			message.success("连接创建成功");
		},
	});
};

const isConnection = computed(() => connectionStore.activeConnectionId);

const activeConnection = computed(() => connectionStore.activeConnection);

const contextMenuVisible = ref(false);
const contextMenuPosition = ref<{ left: number; top: number }>({ left: 0, top: 0 });
const contextMenuTarget = ref<Connection | null>(null);

const hideContextMenu = () => {
	contextMenuVisible.value = false;
	contextMenuTarget.value = null;
};

const showContextMenu = (event: MouseEvent, item: Connection) => {
	event.preventDefault();
	event.stopPropagation();
	contextMenuTarget.value = item;
	contextMenuPosition.value = {
		left: event.clientX,
		top: event.clientY,
	};
	contextMenuVisible.value = true;
};

const handleEditConnection = () => {
	if (!contextMenuTarget.value) return;
	const target = contextMenuTarget.value;
	ConnectionModalRef.value.open({
		title: "编辑连接",
		formData: {
			id: target.id,
			name: target.name,
			host: target.host,
			port: target.port,
			username: target.username ?? "",
			password: target.password ?? "",
			separator: target.separator ?? ":",
			db: target.db ?? 0,
		},
		onSuccess: (data: ConnectionFormState) => {
			connectionStore.updateConnection(target.id, {
				name: data.name,
				host: data.host,
				port: data.port,
				username: data.username ? data.username : null,
				password: data.password ? data.password : null,
				separator: data.separator || ":",
				db: data.db ?? 0,
			});
			message.success("连接已更新");
		},
	});
	hideContextMenu();
};

const handleDeleteConnection = () => {
	if (!contextMenuTarget.value) return;
	const target = contextMenuTarget.value;
	hideContextMenu();
	Modal.confirm({
		title: `确认删除连接「${target.name}」？`,
		okText: "删除",
		cancelText: "取消",
		okType: "danger",
		onOk() {
			connectionStore.deleteConnection(target.id);
			message.success("连接已删除");
		},
	});
};

const handleDisconnectConnection = async () => {
	if (!contextMenuTarget.value) return;
	const target = contextMenuTarget.value;
	hideContextMenu();
	try {
		const res = await disconnectRedis(target.id);
		if (!res.success) {
			message.error(res.message || "断开连接失败");
			return;
		}

		if (connectionStore.activeConnectionId === target.id) {
			connectionStore.setActiveConnection(null);
		}
		message.success("连接已断开");
	} catch (error) {
		console.error("断开连接失败:", error);
		message.error("断开连接失败");
	}
};

const handleReconnectConnection = async () => {
	if (!contextMenuTarget.value) return;
	const target = contextMenuTarget.value;
	hideContextMenu();
	try {
		const res = await connectRedis({
			id: target.id,
			name: target.name,
			host: target.host,
			port: target.port,
			username: target.username ?? undefined,
			password: target.password ?? undefined,
			db: target.db ?? undefined,
		});
		if (!res.success) {
			message.error(res.message || "重新连接失败");
			return;
		}

		connectionStore.setActiveConnection(target.id);
		message.success("连接已重新建立");
	} catch (error) {
		console.error("重新连接失败:", error);
		message.error("重新连接失败");
	}
};

onMounted(() => {
	window.addEventListener("click", hideContextMenu);
	window.addEventListener("contextmenu", hideContextMenu);
});

onBeforeUnmount(() => {
	window.removeEventListener("click", hideContextMenu);
	window.removeEventListener("contextmenu", hideContextMenu);
});
</script>

<template>
  <a-dropdown class="ml-24px" trigger="click">
    <IconButton class="flex items-center">
      <div class="leading-0px">
        <a-badge :status="isConnection ? 'success' : 'default'"/>
      </div>
      <div class="mr-5px">
        {{ activeConnection?.name || '新建连接' }}
      </div>
      <img class="size-10px" :src="downOutlined" alt="">
    </IconButton>
    <template #overlay>
      <div>
        <a-menu :selectedKeys="activeConnection ? [activeConnection.id] : []">
          <a-menu-item key="1" @click="newConnection">
            <PlusOutlined/>
            <span class="ml-5px">新建连接</span>
          </a-menu-item>
          <a-menu-item
            v-for="item in connections"
            :key="item.id"
            @click="onClick(item)"
            @contextmenu.stop.prevent="showContextMenu($event, item)"
          >
            {{ item.name }}
          </a-menu-item>
        </a-menu>
      </div>
    </template>
  </a-dropdown>

  <ConnectionContextMenu
    :visible="contextMenuVisible && !!contextMenuTarget"
    :position="contextMenuPosition"
    @reconnect="handleReconnectConnection"
    @disconnect="handleDisconnectConnection"
    @edit="handleEditConnection"
    @delete="handleDeleteConnection"
  />

  <ConnectionModal ref="ConnectionModalRef"/>
</template>

<style scoped>
</style>
