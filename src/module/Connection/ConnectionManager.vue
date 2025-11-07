<script setup lang="ts">
import { ref, computed } from 'vue';
import { PlusOutlined } from '@ant-design/icons-vue';
import ConnectionModal from "@/module/Connection/ConnectionModal.vue";
import downOutlined from "@/assets/svg/down-outlined.svg";
import { Connection, useConnectionStore } from "@/stores/useConnectionStore.ts";
import IconButton from "@/components/IconButton/index.vue";

const connectionStore = useConnectionStore();

const connections = computed(() => connectionStore.connections);

const onClick = (item: Connection) => {
	connectionStore.setActiveConnection(item.id);
};

const ConnectionModalRef = ref();
const newConnection = () => {
	ConnectionModalRef.value.open({
		onSuccess: (data: Connection) => {
			connectionStore.createConnection(data);
		},
	});
};

const isConnection = computed(() => connectionStore.activeConnectionId);

const activeConnection = computed(() => connectionStore.activeConnection);
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
        <a-menu>
          <a-menu-item key="1" @click="newConnection">
            <PlusOutlined/>
            <span class="ml-5px">新建连接</span>
          </a-menu-item>
          <a-menu-item @click="onClick(item)" v-for="item in connections" :key="item.id">
            {{ item.name }}
          </a-menu-item>
        </a-menu>
      </div>
    </template>
  </a-dropdown>

  <ConnectionModal ref="ConnectionModalRef"/>
</template>

