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
      'bg-[var(--color-primary-bg)]': menuItemProps.active,
      'hover:bg-[var(--color-fill-tertiary)]': !menuItemProps.active
    }"
  >
    <component
      :is="menuItemProps.item.icon"
      class="font-size-18px"
      :class="{
        'text-[var(--color-primary)]': menuItemProps.active,
        'text-[var(--color-text)]': !menuItemProps.active
      }"
    />
  </IconButton>
</template>
