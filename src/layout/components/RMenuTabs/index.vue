<script setup lang="ts">
import {
  BlockOutlined,
  PieChartOutlined,
  GithubOutlined,
} from "@ant-design/icons-vue";
import MenuTabs from "@/components/MeuTabs/index.vue";
import { MenuItemProps } from "@/components/MeuTabs/types.ts";
import {openUrl} from "@tauri-apps/plugin-opener";

const menuList: MenuItemProps[] = [
	{
		id: 1,
		position: "before",
		icon: BlockOutlined,
	},
	{
		id: 2,
		position: "before",
		icon: PieChartOutlined,
	},
  {
    id: 3,
    tooltip: '源码地址',
    position: "after",
    icon: GithubOutlined,
  },
];

const activeBefore = ref();

const handleClick = (item: MenuItemProps) => {
  if(item.id === 3) {
    openUrl('https://github.com/antfl/byte-redis')
  }
}
</script>

<template>
  <MenuTabs
    @item-click="handleClick"
    class="right-menu-tabs"
    :menuList="menuList"
    v-model:before="activeBefore"
   />
</template>

<style scoped lang="less">
.right-menu-tabs {
  border-left: 1px solid var(--color-border-secondary);
}
</style>
