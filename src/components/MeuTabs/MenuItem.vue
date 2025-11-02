<script setup lang="ts">
import type { PropType } from "vue";
import type { MenuItemProps } from "./types";
import IconButton from "@/components/IconButton/index.vue";

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
  <IconButton
    @click="emit('click')"
    :tooltip="menuItemProps.item?.tooltip"
    class="size-30px"
    :class="{
      'bg-#f0f0f0': menuItemProps.active,
      'hover:bg-#f5f5f5': !menuItemProps.active
    }"
  >
    <component
      :is="menuItemProps.item.icon"
      class="font-size-18px"
      :class="{
        'text-#1890ff': menuItemProps.active,
        'text-#1e1f22': !menuItemProps.active
      }"
    />
  </IconButton>
</template>
