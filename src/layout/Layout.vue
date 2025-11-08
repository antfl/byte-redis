<script setup>
import { watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { storeToRefs } from "pinia";
import TitleBar from "@/layout/components/TitleBar/index.vue";
import ViewFooter from "@/layout/components/ViewFooter/index.vue";
import RMenuTabs from "@/layout/components/RMenuTabs/index.vue";
import RedisKeys from "@/module/RedisKeys/RedisKeys.vue";
import { useConnectionStore } from "@/stores/useConnectionStore.ts";

const router = useRouter();
const route = useRoute();
const connectionStore = useConnectionStore();
const { currentKey } = storeToRefs(connectionStore);

watch(
  currentKey,
  (key) => {
    if (key) {
      if (route.path !== "/data") {
        router.replace("/data");
      }
    } else if (route.path === "/data") {
      router.replace("/stats");
    }
  },
  { immediate: true },
);
</script>

<template>
  <a-layout>
    <a-layout-header class="h-[var(--title-bar-height)]!">
      <TitleBar/>
    </a-layout-header>
    <a-layout class="h-[calc(100vh_-_var(--title-bar-height)_-_var(--view-footer-height))] overflow-hidden">
      <a-layout-sider theme="light" width="300">
        <div class="flex h-full overflow-hidden">
          <RedisKeys/>
        </div>
      </a-layout-sider>
      <a-layout-content>
        <div class="flex h-full justify-between overflow-hidden">
          <router-view/>
          <RMenuTabs/>
        </div>
      </a-layout-content>
    </a-layout>
    <ViewFooter/>
  </a-layout>
</template>
