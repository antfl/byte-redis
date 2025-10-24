<script setup lang="ts">
import {
	ApartmentOutlined,
	GithubOutlined,
	PieChartOutlined,
} from "@ant-design/icons-vue";
import ExportData from "@/module/ExportData/index.vue";
import ImportData from "@/module/ImportData/index.vue";
import MenuTabs from "@/components/MeuTabs/index.vue";
import { MenuItemProps } from "@/components/MeuTabs/types.ts";

const menuList: MenuItemProps[] = [
	{
		id: 1,
		position: "before",
		icon: ApartmentOutlined,
		tooltip: "转为树结构",
	},
	{
		id: 2,
		position: "before",
		icon: PieChartOutlined,
	},
	{
		id: 3,
		position: "after",
		icon: GithubOutlined,
	},
];

const currentIndex = ref(1);

const ExportDataRef = ref();
const exportData = () => {
	ExportDataRef.value.use({});
};

const ImportDataRef = ref();
const importData = () => {
	ImportDataRef.value.use({});
};
</script>

<template>
  <div class="left-menu-tabs">
    <MenuTabs
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
