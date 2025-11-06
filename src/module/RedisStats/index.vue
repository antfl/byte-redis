<template>
  <div class="w-full p-10px bg-[var(--color-bg-container)] h-full overflow-auto">
    <div class="mb-12px grid grid-cols-4 gap-12px">
      <a-card size="small" title="连接信息">
        <div>版本：{{ serverInfo.version || '-' }}</div>
        <div>角色：{{ serverInfo.role || '-' }}</div>
        <div>Uptime：{{ formatUptime(serverInfo.uptime) }}</div>
      </a-card>
      <a-card size="small" title="性能">
        <div>OPS：{{ serverInfo.ops_per_sec ?? '-' }}</div>
        <div>命中率：{{ serverInfo.hit_rate != null ? (serverInfo.hit_rate.toFixed ? serverInfo.hit_rate.toFixed(1) : serverInfo.hit_rate) + '%' : '-' }}</div>
        <div>CPU 使用：{{ serverInfo.used_cpu ?? '-' }}</div>
      </a-card>
      <a-card size="small" title="内存">
        <div>使用：{{ formatBytes(serverInfo.memory_usage) }}</div>
        <div>上限：{{ formatBytes(serverInfo.maxmemory) }}</div>
        <div>碎片率：{{ serverInfo.mem_fragmentation_ratio ?? '-' }}</div>
      </a-card>
      <a-card size="small" title="客户端">
        <div>连接数：{{ serverInfo.connections ?? '-' }}</div>
        <div>阻塞：{{ serverInfo.clients_blocked ?? '-' }}</div>
        <div>复制：{{ serverInfo.replication_status || '-' }}</div>
      </a-card>
    </div>

    <div class="grid grid-cols-2 gap-12px">
      <a-card size="small" title="每秒操作趋势">
        <div ref="opsChartRef" style="height: 280px;" />
      </a-card>
      <a-card size="small" title="各 DB Key 数量">
        <div ref="dbChartRef" style="height: 280px;" />
      </a-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import * as echarts from 'echarts/core';
import { LineChart, BarChart } from 'echarts/charts';
import { GridComponent, TooltipComponent, LegendComponent, TitleComponent } from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import { invoke } from "@tauri-apps/api/core";
import { message } from 'ant-design-vue';
import { useConnectionStore } from "@/stores/useConnectionStore.ts";

echarts.use([LineChart, BarChart, GridComponent, TooltipComponent, LegendComponent, TitleComponent, CanvasRenderer]);

const connectionStore = useConnectionStore();

const serverInfo = reactive<Record<string, any>>({});
const opsData = ref<number[]>([]);
const opsLabels = ref<string[]>([]);
const dbSeries = ref<{ name: string; value: number }[]>([]);

const opsChartRef = ref<HTMLDivElement | null>(null);
const dbChartRef = ref<HTMLDivElement | null>(null);
let opsChart: echarts.ECharts | null = null;
let dbChart: echarts.ECharts | null = null;
let timer: any = null;

// 命中率已由后端计算，直接读取 serverInfo.hit_rate

const formatBytes = (bytes?: number) => {
  if (bytes === undefined || bytes === null) return '-';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
};

const formatUptime = (secs?: number) => {
  if (!secs) return '-';
  const d = Math.floor(secs / 86400);
  const h = Math.floor((secs % 86400) / 3600);
  const m = Math.floor((secs % 3600) / 60);
  return `${d}天 ${h}小时 ${m}分`;
};

const initCharts = () => {
  if (opsChartRef.value && !opsChart) {
    opsChart = echarts.init(opsChartRef.value);
    opsChart.setOption({
      tooltip: { trigger: 'axis' },
      xAxis: { type: 'category', data: opsLabels.value },
      yAxis: { type: 'value' },
      series: [{ type: 'line', smooth: true, name: 'OPS', data: opsData.value }],
    });
  }
  if (dbChartRef.value && !dbChart) {
    dbChart = echarts.init(dbChartRef.value);
    dbChart.setOption({
      tooltip: { trigger: 'item' },
      xAxis: { type: 'category', data: dbSeries.value.map(i => i.name) },
      yAxis: { type: 'value' },
      series: [{ type: 'bar', name: 'keys', data: dbSeries.value.map(i => i.value) }],
    });
  }
};

const updateCharts = () => {
  if (opsChart) {
    opsChart.setOption({ xAxis: { data: opsLabels.value }, series: [{ data: opsData.value }] });
  }
  if (dbChart) {
    dbChart.setOption({ xAxis: { data: dbSeries.value.map(i => i.name) }, series: [{ data: dbSeries.value.map(i => i.value) }] });
  }
};

const poll = async () => {
  try {
    if (!connectionStore.activeConnection?.id) return;
    const info = await invoke<Record<string, any>>("get_redis_server_info", {
      connectionId: connectionStore.activeConnection.id,
    });
    Object.assign(serverInfo, info || {});

    const ops = Number(serverInfo.ops_per_sec ?? 0);
    const now = new Date();
    const label = `${now.getHours()}:${String(now.getMinutes()).padStart(2, '0')}:${String(now.getSeconds()).padStart(2, '0')}`;
    opsLabels.value.push(label);
    opsData.value.push(ops);
    if (opsLabels.value.length > 30) {
      opsLabels.value.shift();
      opsData.value.shift();
    }

    const dbCount: number = await invoke("get_db_count", { connectionId: connectionStore.activeConnection.id });
    const series: { name: string; value: number }[] = [];
    for (let i = 0; i < dbCount; i++) {
      const count: number = await invoke("get_db_key_count", { connectionId: connectionStore.activeConnection.id, dbIndex: i });
      series.push({ name: `db${i}`, value: count });
    }
    dbSeries.value = series;

    updateCharts();
  } catch (e: any) {
    message.error(`获取统计信息失败: ${e}`);
  }
};

onMounted(() => {
  initCharts();
  timer = setInterval(poll, 3000);
  poll();
  window.addEventListener('resize', resize);
});

onBeforeUnmount(() => {
  clearInterval(timer);
  window.removeEventListener('resize', resize);
  opsChart?.dispose();
  dbChart?.dispose();
});

const resize = () => {
  opsChart?.resize();
  dbChart?.resize();
}
</script>
