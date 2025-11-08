<script setup lang="ts">

const iconButtonProps = defineProps({
  tooltip: {
    type: [String, Number],
    default: null,
  },
  placement: {
    type: String,
    default: 'right',
  }
});
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
