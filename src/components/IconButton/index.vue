<script setup lang="ts">
import type { TooltipPlacement } from 'ant-design-vue/es/tooltip/abstractTooltipProps';

const iconButtonProps = withDefaults(
  defineProps<{
    tooltip?: string | number | null;
    placement?: TooltipPlacement;
  }>(),
  {
    tooltip: null,
    placement: 'right',
  },
);
</script>

<template>
  <a-tooltip :placement="iconButtonProps.placement" :title="iconButtonProps.tooltip">
    <div
      class="cursor-pointer rd-3px px-8px h-28px flex items-center justify-center"
      :class="{
      'hover:bg-[var(--color-fill-tertiary)]': true
    }"
    >
      <slot/>
    </div>
  </a-tooltip>
</template>
