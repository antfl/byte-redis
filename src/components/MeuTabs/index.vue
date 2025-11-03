<script setup lang="ts">
import MenuItem from "./MenuItem.vue";
import type { MenuItemProps } from "./types";
import { computed } from "vue";

const props = defineProps({
	menuList: {
		type: Array as () => MenuItemProps[],
		default: () => [],
	},
	before: {
		type: [String, Number] as PropType<string | number | null>,
		default: null,
	},
	after: {
		type: [String, Number] as PropType<string | number | null>,
		default: null,
	},
});

const beforeList = computed(() =>
	props.menuList.filter((menuItem) => menuItem.position === "before"),
);
const afterList = computed(() =>
	props.menuList.filter((menuItem) => menuItem.position === "after"),
);

// 定义事件
const emit = defineEmits<{
	(
		e: "item-click",
		item: MenuItemProps,
		index: number,
		type: "before" | "after",
	): void;
	(e: "update:before", id: string | number | null): void;
	(e: "update:after", id: string | number | null): void;
}>();

// 处理菜单点击
const handleItemClick = (
	item: MenuItemProps,
	index: number,
	type: "before" | "after",
) => {
	if (type === "before") {
		// 如果点击的是当前已选中的 before 项，则取消选中
		if (props.before === item.id) {
			emit("update:before", null);
		} else {
			emit("update:before", item.id);
		}
	} else {
		// 如果点击的是当前已选中的 after 项，则取消选中
		if (props.after === item.id) {
			emit("update:after", null);
		} else {
			emit("update:after", item.id);
		}
	}

	emit("item-click", item, index, type);
};
</script>

<template>
  <div class="menu-tabs w-40px h-full py-5px bg-[var(--color-bg-container)] flex justify-between flex-col">
    <a-space :size="10" align="center" direction="vertical">
      <MenuItem
        v-for="(item, index) in beforeList"
        :key="item.id"
        :item="item"
        :active="item.id === props.before"
        @click="handleItemClick(item, index, 'before')"
      />
    </a-space>

    <a-space :size="10" align="center" direction="vertical">
      <MenuItem
        v-for="(item, index) in afterList"
        :key="item.id"
        :item="item"
        :active="item.id === props.after"
        @click="handleItemClick(item, index, 'after')"
      />
    </a-space>
  </div>
</template>
