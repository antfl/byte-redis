<script setup lang="ts">
import type { PropType } from "vue";
import type { MenuItemProps } from "./types";

const menuItemProps = defineProps({
	item: {
		type: Object as PropType<MenuItemProps>,
		required: true,
	},
	active: {
		type: Boolean,
		default: false,
	},
});

const emit = defineEmits(["click"]);
</script>

<template>
  <a-tooltip placement="right">
    <template #title>
      <span>{{menuItemProps.item?.tooltip}}</span>
    </template>
    <div
      class="menu-item cursor-pointer rd-3px size-30px flex items-center justify-center"
      :class="{
      'bg-#f0f0f0': menuItemProps.active,
      'hover:bg-#f5f5f5': !menuItemProps.active
    }"
      @click="emit('click')"
    >
      <component
        :is="menuItemProps.item.icon"
        class="font-size-18px"
        :class="{
        'text-#1890ff': menuItemProps.active,
        'text-#1e1f22': !menuItemProps.active
      }"
      />
    </div>
  </a-tooltip>
</template>
