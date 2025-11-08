<script setup lang="ts">
import {ref, computed, watch} from 'vue';
import {message} from "ant-design-vue";
import database from "@/assets/svg/database.svg";
import downOutlined from "@/assets/svg/down-outlined.svg";
import IconButton from "@/components/IconButton/index.vue";
import {useConnectionStore} from "@/stores/useConnectionStore.ts";
import {connectRedis, getDbCount, getDbKeyCount, getAllDbKeyCounts, selectDb} from "@/api";

interface DatabaseInfo {
  index: number;
  keyCount: number;
}

const connectionStore = useConnectionStore();
const activeConnection = computed(() => connectionStore.activeConnection);
const currentDbIndex = computed(() => connectionStore.currentDbIndex);

const databases = ref<DatabaseInfo[]>([]);
const loading = ref(false);
const lastConnectedId = ref<string | null>(null);

const ensureRedisConnection = async (): Promise<boolean> => {
  if (!activeConnection.value) {
    return false;
  }

  const {
    id,
    name,
    host,
    port,
    username,
    password,
    db,
  } = activeConnection.value;
  if (!id) {
    return false;
  }

  if (lastConnectedId.value === id) {
    return true;
  }

  try {
		const connectRes = await connectRedis({
			id,
			name,
			host,
			port,
			username: username ?? undefined,
			password: password ?? undefined,
			db: db ?? undefined,
		});

    if (!connectRes.success) {
      message.error(connectRes.message || "连接 Redis 失败");
      return false;
    }

    lastConnectedId.value = id;
    return true;
  } catch (error) {
    console.error("连接 Redis 失败:", error);
    message.error("连接 Redis 失败");
    return false;
  }
};

// 获取数据库数量
const fetchDbCount = async (): Promise<number> => {
  if (!activeConnection.value?.id) {
    return 16; // 默认16个数据库
  }

  try {
    const res = await getDbCount(activeConnection.value.id);
    if (!res.success || res.data === undefined) {
      message.error(res.message || "获取数据库数量失败");
      return 16;
    }
    return res.data;
  } catch (error) {
    console.error("获取数据库数量失败:", error);
    message.error("获取数据库数量失败");
    return 16;
  }
};

// 获取数据库键数量
const fetchDbKeyCount = async (dbIndex: number): Promise<number> => {
  if (!activeConnection.value?.id) {
    return 0;
  }

  try {
    const res = await getDbKeyCount(activeConnection.value.id, dbIndex);
    if (!res.success || res.data === undefined) {
      return 0;
    }
    return res.data;
  } catch (error) {
    console.error(`获取数据库 DB${dbIndex.toString().padStart(2, "0")} 键数量失败:`, error);
    return 0;
  }
};

// 加载数据库列表
const loadDatabases = async () => {
  if (!activeConnection.value?.id) {
    databases.value = [];
    return;
  }

  const connected = await ensureRedisConnection();
  if (!connected) {
    return;
  }

  loading.value = true;
  try {
    const dbCount = await fetchDbCount();

    // 使用批量接口获取所有数据库的键数量
    try {
      const resultsRes = await getAllDbKeyCounts(activeConnection.value.id, dbCount);
      if (!resultsRes.success || !resultsRes.data) {
        throw new Error(resultsRes.message || "批量获取失败");
      }

      databases.value = resultsRes.data.map((item) => ({
        index: item.db_index,
        keyCount: item.key_count,
      }));
    } catch (error) {
      // 如果批量接口失败，回退到逐个获取
      console.warn("批量获取失败，使用逐个获取:", error);
      const promises = Array.from({length: dbCount}, (_, i) =>
        fetchDbKeyCount(i).then((keyCount) => ({
          index: i,
          keyCount,
        })),
      );
      databases.value = await Promise.all(promises);
    }
  } catch (error) {
    console.error("加载数据库列表失败:", error);
    message.error("加载数据库列表失败");
  } finally {
    loading.value = false;
  }
};

// 切换数据库
const selectDatabase = async (dbIndex: number) => {
  if (!activeConnection.value?.id) {
    message.warning("请先连接 Redis");
    return;
  }

  if (currentDbIndex.value === dbIndex) {
    return;
  }

  try {
    const res = await selectDb(activeConnection.value.id, dbIndex);
    if (!res.success) {
      message.error(res.message || "切换数据库失败");
      return;
    }

    connectionStore.setCurrentDbIndex(dbIndex);
    message.success(`已切换到 DB${dbIndex.toString().padStart(2, "0")}`);

    // 刷新键列表（通过触发通知，但不重新加载数据库列表）
    connectionStore.notify();
  } catch (error) {
    console.error("切换数据库失败:", error);
    message.error("切换数据库失败");
  }
};

// 格式化数据库名称
const formatDbName = (index: number): string => {
  return `DB${index.toString().padStart(2, "0")}`;
};

// 监听连接变化
watch(
  () => activeConnection.value?.id,
  (newId, oldId) => {
    // 只在连接真正变化时加载
    if (newId !== oldId) {
      lastConnectedId.value = null;
      loadDatabases();
    }
  },
  {immediate: true},
);
</script>

<template>
  <a-dropdown
    class="ml-24px"
    trigger="click"
    :disabled="!activeConnection"
  >
    <IconButton>
      <img class="size-24px" :src="database" alt="">
      <div class="mr-5px">
        {{ activeConnection ? formatDbName(currentDbIndex) : "未连接" }}
      </div>
      <img class="size-10px" :src="downOutlined" alt="">
    </IconButton>
    <template #overlay>
      <a-menu
        v-if="activeConnection"
        :selectedKeys="[currentDbIndex.toString()]"
      >
        <template v-if="loading">
          <a-menu-item>
            <div class="flex justify-center">
              <a-spin :spinning="true"/>
            </div>
          </a-menu-item>
        </template>
        <template v-else>
          <a-menu-item
            v-for="db in databases"
            :key="db.index.toString()"
            @click="selectDatabase(db.index)"
          >
            <span class="color-#1e1f22">{{ formatDbName(db.index) }}</span>
            <span class="ml-8px">[{{ db.keyCount }}]</span>
          </a-menu-item>
        </template>
      </a-menu>
      <a-menu v-else>
        <a-menu-item disabled>
          <span>请先连接 Redis</span>
        </a-menu-item>
      </a-menu>
    </template>
  </a-dropdown>
</template>

