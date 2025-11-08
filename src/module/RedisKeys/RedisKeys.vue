<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue';
import {
  CloseOutlined,
  SearchOutlined,
  PlusOutlined,
  ReloadOutlined,
  SwapOutlined,
  ArrowUpOutlined,
  ArrowDownOutlined,
  DownOutlined,
} from '@ant-design/icons-vue';
import { storeToRefs } from "pinia";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";
import { connectRedis, getKeys, setKey } from "@/api";
import AddKeyModal from "@/module/AddKeyModal/AddKeyModal.vue";
import ExportData from "@/module/ExportData/ExportData.vue";
import ImportData from "@/module/ImportData/ImportData.vue";
import { message } from "ant-design-vue";
import type { TreeProps } from "ant-design-vue";
import IconButton from "@/components/IconButton/index.vue";
import { getTypeColor } from '@/utils/format'

interface KeyItem {
	key: string;
	type: string;
}

interface TreeNode {
	key: string;
	title: string;
	children?: TreeNode[];
	isLeaf?: boolean;
	value?: KeyItem;
	icon?: any;
	length?: number;
	root?: boolean;
	selectable?: boolean;
}

interface ConnectionConfig {
	host: string;
	name: string;
	id: string;
	username?: string;
	password?: string;
	port: number;
	separator?: string;
}

interface AddKeyFormData {
	key: string;
	type: string;
	ttl?: number;
	stringValue?: string;
	hashValue?: Array<{ field: string; value: string }>;
	listValue?: string[];
	setValue?: string[];
	zsetValue?: Array<{ score: number; value: string }>;
}


const connectionStore = useConnectionStore();
const { activeConnectionId, currentDbIndex, activeConnection, keyListRefreshTrigger } = storeToRefs(connectionStore);
const filterText = ref("");
const isLoading = ref(true);
const fullTreeData = ref<TreeNode[]>([]);
const rawKeys = ref<KeyItem[]>([]);
const useSeparator = ref(true);
const currentSeparator = computed(() => activeConnection.value?.separator || ":");

const genData = computed(() => {
	if (!filterText.value) return fullTreeData.value;

	const searchLower = filterText.value.toLowerCase();

	const filterNode = (node: TreeNode): TreeNode | null => {
		if (node.title.toLowerCase().includes(searchLower)) {
			return node;
		}

		if (node.children && node.children.length > 0) {
			const filteredChildren = node.children
				.map(filterNode)
				.filter(Boolean) as TreeNode[];

			if (filteredChildren.length > 0) {
				return {
					...node,
					children: filteredChildren,
				};
			}
		}

		return null;
	};

	return fullTreeData.value.map(filterNode).filter(Boolean) as TreeNode[];
});

watch(
	activeConnectionId,
	() => {
		if (
			activeConnection.value &&
			activeConnectionId.value
		) {
			useSeparator.value = true;
			nextTick(() => {
				init(activeConnection.value as ConnectionConfig);
			});
		}
	},
	{
		immediate: true,
	},
);

watch(
	currentDbIndex,
	(newIndex, oldIndex) => {
		if (
			newIndex !== oldIndex &&
			activeConnection.value &&
			activeConnectionId.value
		) {
			nextTick(() => {
				init(activeConnection.value as ConnectionConfig, true);
			});
		}
	},
);

// 监听 key 列表刷新触发器
watch(
	keyListRefreshTrigger,
	() => {
		if (activeConnection.value && activeConnectionId.value) {
			nextTick(() => {
				init(activeConnection.value as ConnectionConfig, true);
			});
		}
	},
);

const init = async (config: ConnectionConfig, skipConnect = false) => {
	if (!config) {
		return;
	}
	connectionStore.setCurrentKey(null);
	connectionStore.setCurrentKeyCount(0);
	isLoading.value = true;
	fullTreeData.value = [];
	try {
		if (!skipConnect) {
			const connectRes = await connectRedis({
				name: config.name,
				id: config.id,
				host: config.host,
				username: config.username,
				password: config.password,
				port: config.port,
			});
			if (!connectRes.success) {
				message.error(connectRes.message);
				return;
			}
		}

		const keysRes = await getKeys(config.id, "*");
		if (!keysRes.success || !keysRes.data) {
			connectionStore.setCurrentKeyCount(0);
			rawKeys.value = [];
			fullTreeData.value = [];
			message.error(keysRes.message || "获取键列表失败");
			return;
		}

		connectionStore.setCurrentKeyCount(keysRes.data.total ?? keysRes.data.keys.length ?? 0);
		rawKeys.value = keysRes.data.keys ?? [];
		applyKeyTree();
		setTreeHeight();
	} catch (error) {
		console.error("初始化失败:", error);
		message.error(`初始化失败: ${error}`);
	} finally {
		isLoading.value = false;
	}
};

const applyKeyTree = () => {
	fullTreeData.value = buildKeyTree(rawKeys.value, useSeparator.value ? currentSeparator.value : null);
};

const handleNodeClick = (_event: MouseEvent, node: any) => {
	const rawNode: TreeNode | undefined = node?.rawNode || node;
	if (rawNode?.value?.key) {
		connectionStore.setCurrentKey(rawNode.value.key);
	}
};

watch([useSeparator, currentSeparator], () => {
	if (!rawKeys.value.length) {
		fullTreeData.value = [];
		return;
	}
	applyKeyTree();
});

function buildKeyTree(keys: KeyItem[], separator: string | null = ":"): TreeNode[] {
	const root: TreeNode = {
		key: "root",
		title: "ROOT",
		children: [],
		isLeaf: false,
	};

	const keyMap: Record<string, TreeNode> = {};
	const shouldSplit = !!separator;

	if (!shouldSplit) {
		const flatNodes = keys.map((item) => ({
			key: item.key,
			title: item.key,
			children: undefined,
			isLeaf: true,
			value: item,
			root: true,
			selectable: true,
		}));

		flatNodes.sort((a, b) => a.title.localeCompare(b.title));

		return flatNodes;
	}

	keys.forEach((item) => {
		const parts = item.key.split(separator);
		let currentNode = root;
		let currentPath = "";

		for (let i = 0; i < parts.length; i++) {
			const part = parts[i];
			currentPath = currentPath ? `${currentPath}${separator}${part}` : part;

			if (!keyMap[currentPath]) {
				const isLeaf = i === parts.length - 1;

				const newNode: TreeNode = {
					key: currentPath,
					title: part,
					children: isLeaf ? undefined : [],
					isLeaf,
					value: isLeaf ? item : undefined,
					root: i === 0,
					selectable: isLeaf,
				};

				keyMap[currentPath] = newNode;
				if (currentNode.children) {
					currentNode.children.push(newNode);
				}
			}

			currentNode = keyMap[currentPath];
		}
	});

	const calculateLength = (node: TreeNode): number => {
		if (node.isLeaf) {
			return 0;
		}

		(node.children || []).forEach(calculateLength);

		const length = node.children?.length || 0;

		node.length = length;

		if (node.children && node.children.length > 0) {
			node.children.sort((a, b) => {
				const diff = (b.length || 0) - (a.length || 0);
				return diff !== 0 ? diff : a.title.localeCompare(b.title);
			});
		}

		return length;
	};

	calculateLength(root);

	return root.children || [];
}

const selectKey: TreeProps['onSelect'] = (_selectedKeys, info) => {
	const rawNode: TreeNode | undefined = (info.node as any)?.rawNode || info.node;

	if (rawNode?.value?.key) {
		connectionStore.setCurrentKey(rawNode.value.key);
	}
};

const AddKeyModalRef = ref();
const ExportDataRef = ref();
const ImportDataRef = ref();

const showAddKeyModal = () => {
	AddKeyModalRef.value?.open({
		onSuccess: async (data: AddKeyFormData) => {
			try {
				let value: any;
				switch (data.type) {
					case "string":
						value = data.stringValue || "";
						break;
					case "hash":
						// 将 Array<{ field: string; value: string }> 转换为 Record<string, string>
						const hashObj: Record<string, string> = {};
						(data.hashValue || []).forEach((item) => {
							if (item.field && item.value) {
								hashObj[item.field] = item.value;
							}
						});
						value = hashObj;
						break;
					case "list":
						value = data.listValue || [];
						break;
					case "set":
						value = data.setValue || [];
						break;
					case "zset":
						// zset 需要转换为 [["member", score], ...] 格式
						// data.zsetValue 是 { score: number, value: string }[] 格式
						value = (data.zsetValue || []).map((item) => [item.value, item.score]);
						break;
				}

				const result = await setKey(
					activeConnectionId.value!,
					data.key,
					data.type,
					value,
					data.ttl || 0,
				);

				if (result.success) {
					message.success(result.message);
					// 刷新键列表
					if (activeConnection.value) {
						init(activeConnection.value as ConnectionConfig, true);
					}
				} else {
					message.error(result.message);
				}
			} catch (error) {
				message.error(`添加键失败: ${error}`);
			}
		},
	});
};

const RedisKeyTreeRef = ref<HTMLElement>();
const redisKeyTreeHeight = ref(200);
const setTreeHeight = () => {
	nextTick(() => {
		if (RedisKeyTreeRef.value) {
			redisKeyTreeHeight.value = RedisKeyTreeRef.value.clientHeight;
		}
	});
};

const loadKeys = () => {
  if (activeConnection.value) {
    init(activeConnection.value as ConnectionConfig, true);
  }
};

const showExportModal = () => {
	ExportDataRef.value?.open();
};

const showImportModal = () => {
	ImportDataRef.value?.open();
};
</script>

<template>
  <div class="redis-key-container w-full flex flex-col">
    <div class="redis-key-filter h-40px flex items-center">
      <a-input
          class="pr-0"
          :bordered="false"
          v-model:value="filterText"
          placeholder="搜索"
          allow-clear
        >
          <template #clearIcon	>
            <IconButton class="size-20px!">
              <CloseOutlined class="font-size-11px color-#1e1e1e"/>
            </IconButton>
          </template>
          <template #prefix>
            <SearchOutlined/>
          </template>
          <template #suffix>

            <IconButton placement="bottom" tooltip="新增" class="size-24px! mr-0!" @click="showAddKeyModal">
              <PlusOutlined class="font-size-16px"/>
            </IconButton>
            <IconButton placement="bottom" tooltip="刷新" class="size-24px! mr-0px!" @click="loadKeys">
              <ReloadOutlined />
            </IconButton>
            <IconButton class="mr-4px" @click.stop="useSeparator = !useSeparator">
              <ApartmentOutlined v-if="useSeparator"/>
              <MenuOutlined v-else/>
            </IconButton>
          </template>
        </a-input>
    </div>

    <div class="flex-1" ref="RedisKeyTreeRef">
      <a-spin v-if="isLoading" class="pt-32px w-full" :spinning="isLoading"/>
      <a-directory-tree
        v-if="genData.length > 0"
        :tree-data="genData"
        @select="selectKey"
        @click="handleNodeClick"
        :height="redisKeyTreeHeight"
      >
        <template #switcherIcon="{ switcherCls }">
          <DownOutlined :class="switcherCls"/>
        </template>
        <template #title="{ title, value, length ,root}">
          <div class="flex-inline items-center" :class="{'ml-24px': root && !length}">
            <a-tag class="bg-#fff mr-0 font-700 font-size-11px h-auto leading-13px px-3px" v-if="value"  :bordered="false" :style="{color: getTypeColor(value.type), borderColor: getTypeColor(value.type)}">
              {{ value.type.slice(0, 3).toUpperCase() }}
            </a-tag>
            <span class="ml-5px break-all line-clamp-1">{{ title }}
            <span v-if="length">[{{length}}]</span>
            </span>
          </div>
        </template>
      </a-directory-tree>
      <a-empty class="pt-20px" v-else-if="!isLoading" description="未找到键值数据"/>
    </div>
    <div class="keys-action px-8px h-40px flex items-center justify-between">
      <IconButton class="size-24px!" tooltip="迁移数据" placement="top">
        <SwapOutlined />
      </IconButton>
      <div class="flex">
        <IconButton class="size-24px!" tooltip="导入" placement="top" @click="showImportModal">
          <ArrowUpOutlined/>
        </IconButton>
        <IconButton class="size-24px!" tooltip="导出" placement="top" @click="showExportModal">
          <ArrowDownOutlined/>
        </IconButton>
      </div>
    </div>
    <AddKeyModal ref="AddKeyModalRef"/>
    <ExportData ref="ExportDataRef"/>
    <ImportData ref="ImportDataRef"/>
  </div>
</template>

<style scoped lang="less">
.redis-key-container {
  border-right: 1px solid var(--color-border-secondary);

  .redis-key-filter {
    border-bottom: 1px solid var(--color-border-secondary);
  }

  :deep(.ant-tree-switcher-noop) {
    display: none;
  }
  :deep(.ant-tree-iconEle) {
    width: auto;
  }
  :deep(.anticon-file) {
    display: none;
  }
}
.keys-action {
  border-top: 1px solid var(--color-border-secondary);
}

</style>

