<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";
import AddKeyModal from "@/module/AddKeyModal/index.vue";
import { message } from "ant-design-vue";
import IconButton from "@/components/IconButton/index.vue";

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
}

interface ConnectionConfig {
	host: string;
	name: string;
	id: string;
	username?: string;
	password?: string;
	port: number;
}

interface AddKeyFormData {
	key: string;
	type: string;
	ttl?: number;
	stringValue?: string;
	hashValue?: Record<string, string>;
	listValue?: string[];
	setValue?: string[];
	zsetValue?: Array<[string, number]>;
}

interface SetKeyResult {
	success: boolean;
	message: string;
}

const connectionStore = useConnectionStore();
const filterText = ref("");
const isLoading = ref(true);
const fullTreeData = ref<TreeNode[]>([]);

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
	() => connectionStore.activeConnectionId,
	() => {
		if (
			connectionStore.activeConnection &&
			connectionStore.activeConnectionId
		) {
			nextTick(() => {
				init(connectionStore.activeConnection as ConnectionConfig);
			});
		}
	},
	{
		immediate: true,
	},
);

const init = async (config: ConnectionConfig) => {
	if (!config) {
		return;
	}
	isLoading.value = true;
	fullTreeData.value = [];
	try {
		await invoke("connect_redis", {
			config: {
				name: config.name,
				id: config.id,
				host: config.host,
				username: config.username,
				password: config.password,
				port: config.port,
			},
		});

		const { keys } = await invoke<{ keys: KeyItem[] }>("get_keys", {
			connectionId: config.id,
			pattern: "*",
		});

		fullTreeData.value = buildKeyTree(keys, ":");
		setTreeHeight();
	} catch (error) {
		console.error("初始化失败:", error);
	} finally {
		isLoading.value = false;
	}
};

function buildKeyTree(keys: KeyItem[], separator = ":"): TreeNode[] {
	const root: TreeNode = {
		key: "root",
		title: "根节点",
		children: [],
		isLeaf: false,
	};

	const keyMap: Record<string, TreeNode> = {};

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

const selectKey = async (selectedKeys: string[]) => {
	connectionStore.setCurrentKey(selectedKeys[0]);
};

const AddKeyModalRef = ref();

const showAddKeyModal = () => {
	AddKeyModalRef.value?.use({
		onSuccess: async (data: AddKeyFormData) => {
			try {
				let value: any;
				switch (data.type) {
					case "string":
						value = data.stringValue;
						break;
					case "hash":
						value = data.hashValue;
						break;
					case "list":
						value = data.listValue;
						break;
					case "set":
						value = data.setValue;
						break;
					case "zset":
						value = data.zsetValue;
						break;
				}

				const result = await invoke<SetKeyResult>("set_key", {
					connectionId: connectionStore.activeConnectionId,
					key: data.key,
					keyType: data.type,
					value: value,
					ttl: data.ttl,
				});

				if (result.success) {
					message.success(result.message);
				} else {
					message.error(result.message);
				}
			} catch (error) {
				message.error(`添加键失败: ${error}`);
			}
		},
	});
};

const getTypeColor = (type: string) => {
	const colors: Record<string, string> = {
		string: "#1677ff",
		hash: "green",
		list: "orange",
		set: "purple",
		zset: "red",
	};
	return colors[type] || "gray";
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
</script>

<template>
  <div class="redis-key-container w-full flex flex-col">
    <div class="redis-key-filter h-40px flex items-center">
      <div class="w-full">
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
            <IconButton class="size-24px!" @click="showAddKeyModal">
              <PlusOutlined class="font-size-16px"/>
            </IconButton>
          </template>
        </a-input>
      </div>
      <div class="pr-4px flex">
        <IconButton class="size-24px!">
          <ReloadOutlined />
        </IconButton>
        <IconButton class="size-24px!">
          <ArrowUpOutlined/>
        </IconButton>
        <IconButton class="size-24px!">
          <ArrowDownOutlined/>
        </IconButton>
      </div>
    </div>

    <div class="flex-1" ref="RedisKeyTreeRef">
      <a-spin v-if="isLoading" class="pt-32px w-full" :spinning="isLoading"/>
      <a-directory-tree
        v-if="genData.length > 0"
        :tree-data="genData"
        @select="selectKey"
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
      <a-empty v-else-if="!isLoading" description="未找到键值数据"/>
    </div>
    <AddKeyModal ref="AddKeyModalRef"/>
  </div>
</template>

<style scoped lang="less">
.redis-key-container {
  border-right: 1px solid #f0f0f0;

  .redis-key-filter {
    border-bottom: 1px solid #f0f0f0;
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
</style>
