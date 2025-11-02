<script setup lang="ts">
import {
	ApartmentOutlined,
	GithubOutlined,
} from "@ant-design/icons-vue";
import ExportData from "@/module/ExportData/index.vue";
import ImportData from "@/module/ImportData/index.vue";
import MenuTabs from "@/components/MeuTabs/index.vue";
import { MenuItemProps } from "@/components/MeuTabs/types.ts";
import { openUrl } from "@tauri-apps/plugin-opener";

const menuList: MenuItemProps[] = [
	{
		id: 1,
		position: "before",
		icon: ApartmentOutlined,
		tooltip: "切换结构",
	},
	{
		id: 3,
    tooltip: '源码地址',
		position: "after",
		icon: GithubOutlined,
	},
];

const handleClick = (item: MenuItemProps) => {
  if(item.id === 3) {
    openUrl('https://github.com/antfl/byte-redis')
  }
}
</script>

<template>
  <div class="left-menu-tabs">
    <MenuTabs
      @item-click="handleClick"
      :menuList="menuList"
    />
    <ExportData ref="ExportDataRef"/>
    <ImportData ref="ImportDataRef"/>
  </div>
</template>

<style scoped>
.left-menu-tabs {
  border-right: 1px solid #f0f0f0;
}
</style>
