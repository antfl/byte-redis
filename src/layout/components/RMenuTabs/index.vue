<script setup lang="ts">
import {
	BlockOutlined,
	AppstoreOutlined,
	PieChartOutlined,
} from "@ant-design/icons-vue";
import MenuTabs from "@/components/MeuTabs/index.vue";
import { MenuItemProps } from "@/components/MeuTabs/types.ts";

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
		position: "after",
		icon: AppstoreOutlined,
	},
];

const activeAfter = ref();
const activeBefore = ref();
</script>

<template>
  <MenuTabs
    class="right-menu-tabs"
    :menuList="menuList"
    v-model:before="activeBefore"
    v-model:after="activeAfter
"/>
</template>

<style scoped>
.right-menu-tabs {
  border-left: 1px solid #f0f0f0;
}
</style>
