<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { ThemeMode, useThemeStore } from '@/stores/theme';

const themeStore = useThemeStore();
const { themeMode } = storeToRefs(themeStore);
const { toggleTheme } = themeStore;

const themeList: { value: ThemeMode; title: string }[] = [
  {
    value: 'light',
    title: '浅色模式',
  },
  {
    value: 'dark',
    title: '深色模式',
  },
  {
    value: 'auto',
    title: '跟随系统',
  },
];

const toggle = (value: ThemeMode) => {
  toggleTheme(value);
};
</script>

<template>
  <div class="w-400px overflow-hidden">
    <div class="title-label mb-16px font-size-17px">主题模式</div>
    <a-flex :gap="16">
      <a-card
        v-for="item in themeList"
        :key="item.value"
        class="w-full font-700 cursor-pointer"
        @click="toggle(item.value)"
        :class="themeMode === item.value ? 'color-[var(--color-primary)]' : ''"
      >
        {{ item.title }}
      </a-card>
    </a-flex>
  </div>
</template>
