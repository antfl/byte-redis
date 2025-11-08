<template>
  <div class="w-full p-10px bg-[var(--color-bg-container)] h-full overflow-auto">
    <div class="mb-12px grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-12px">
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

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-12px">
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
import { ref, reactive, onMounted, onBeforeUnmount, watch } from 'vue';
import * as echarts from 'echarts/core';
import { LineChart, BarChart } from 'echarts/charts';
import { GridComponent, TooltipComponent, LegendComponent, TitleComponent } from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import { message } from 'ant-design-vue';
import { useConnectionStore } from "@/stores/useConnectionStore.ts";
import { getRedisServerInfo, getDbCount, getDbKeyCount } from "@/api";
import { useThemeStore } from "@/stores/theme.ts";

echarts.use([LineChart, BarChart, GridComponent, TooltipComponent, LegendComponent, TitleComponent, CanvasRenderer]);

const connectionStore = useConnectionStore();
const themeStore = useThemeStore();

const serverInfo = reactive<Record<string, any>>({});
const opsData = ref<number[]>([]);
const opsLabels = ref<string[]>([]);
const dbSeries = ref<{ name: string; value: number }[]>([]);

const opsChartRef = ref<HTMLDivElement | null>(null);
const dbChartRef = ref<HTMLDivElement | null>(null);
let opsChart: echarts.ECharts | null = null;
let dbChart: echarts.ECharts | null = null;
let timer: any = null;
let disposeThemeListener: (() => void) | void;

type RGB = { r: number; g: number; b: number };
const clamp = (value: number) => Math.max(0, Math.min(255, Math.round(value)));
const hexToRgb = (hex: string): RGB => {
  const fallback: RGB = { r: 22, g: 119, b: 255 };
  if (!hex) return fallback;
  let value = hex.replace('#', '');
  if (value.length === 3) {
    value = value
      .split('')
      .map((char) => char + char)
      .join('');
  }
  if (!/^[0-9a-fA-F]{6}$/.test(value)) {
    return fallback;
  }
  const num = parseInt(value, 16);
  return {
    r: (num >> 16) & 255,
    g: (num >> 8) & 255,
    b: num & 255,
  };
};
const lighten = (color: RGB, amount: number): RGB => ({
  r: clamp(color.r + (255 - color.r) * amount),
  g: clamp(color.g + (255 - color.g) * amount),
  b: clamp(color.b + (255 - color.b) * amount),
});
const darken = (color: RGB, amount: number): RGB => ({
  r: clamp(color.r * (1 - amount)),
  g: clamp(color.g * (1 - amount)),
  b: clamp(color.b * (1 - amount)),
});
const toRgba = (color: RGB, alpha = 1) =>
  `rgba(${color.r}, ${color.g}, ${color.b}, ${Math.min(1, Math.max(0, alpha))})`;

const getChartPalette = () => {
  const mode = themeStore.currentTheme;
  const isDark = mode === 'dark';
  const primaryHex = themeStore.primaryColor || '#1677ff';
  const primaryRgb = hexToRgb(primaryHex);
  const areaBase = isDark ? darken(primaryRgb, 0.15) : primaryRgb;
  return {
    textColor: isDark ? '#ECEFF4' : '#1F1F1F',
    subTextColor: isDark ? '#9CA3AF' : '#5F6B7A',
    axisLineColor: isDark ? 'rgba(148, 163, 184, 0.35)' : 'rgba(148, 163, 184, 0.5)',
    splitLineColor: isDark ? 'rgba(148, 163, 184, 0.14)' : 'rgba(148, 163, 184, 0.2)',
    tooltipBackground: isDark ? 'rgba(17, 24, 39, 0.92)' : '#FFFFFF',
    tooltipBorder: isDark ? 'rgba(148, 163, 184, 0.25)' : 'rgba(148, 163, 184, 0.32)',
    primaryColor: primaryHex,
    areaColor: toRgba(areaBase, isDark ? 0.18 : 0.2),
    barColorStart: toRgba(lighten(primaryRgb, isDark ? 0.08 : 0.25), 0.95),
    barColorEnd: toRgba(darken(primaryRgb, isDark ? 0.05 : 0.1), 0.85),
  };
};

const buildOpsOption = () => {
  const palette = getChartPalette();
  const axisLabelStyle = {
    color: palette.subTextColor,
    fontSize: 12,
  };

  return {
    backgroundColor: 'transparent',
    grid: { left: 48, right: 24, top: 36, bottom: 30, containLabel: true },
    tooltip: {
      trigger: 'axis',
      backgroundColor: palette.tooltipBackground,
      borderColor: palette.tooltipBorder,
      textStyle: { color: palette.textColor },
      axisPointer: {
        lineStyle: { color: palette.primaryColor, width: 1.2 },
      },
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: opsLabels.value,
      axisLabel: axisLabelStyle,
      axisLine: { lineStyle: { color: palette.axisLineColor, width: 1 } },
      axisTick: { show: false },
      splitLine: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLabel: axisLabelStyle,
      axisLine: { show: false },
      splitLine: { lineStyle: { color: palette.splitLineColor, type: 'dashed' } },
      axisTick: { show: false },
    },
    series: [
      {
        type: 'line',
        name: 'OPS',
        data: opsData.value,
        smooth: true,
        showSymbol: false,
        symbol: 'circle',
        lineStyle: { color: palette.primaryColor, width: 2 },
        itemStyle: { color: palette.primaryColor },
        areaStyle: { color: palette.areaColor },
      },
    ],
  };
};

const buildDbOption = () => {
  const palette = getChartPalette();
  const categories = dbSeries.value.map((item) => item.name.toUpperCase());
  const axisLabelStyle = {
    color: palette.subTextColor,
    fontSize: 12,
  };
  return {
    backgroundColor: 'transparent',
    grid: { left: 56, right: 24, top: 36, bottom: 36, containLabel: true },
    tooltip: {
      trigger: 'axis',
      backgroundColor: palette.tooltipBackground,
      borderColor: palette.tooltipBorder,
      textStyle: { color: palette.textColor },
      axisPointer: { type: 'shadow' },
    },
    xAxis: {
      type: 'category',
      data: categories,
      axisLabel: axisLabelStyle,
      axisLine: { lineStyle: { color: palette.axisLineColor, width: 1 } },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLabel: axisLabelStyle,
      axisLine: { show: false },
      axisTick: { show: false },
      splitLine: { lineStyle: { color: palette.splitLineColor, type: 'dashed' } },
    },
    series: [
      {
        type: 'bar',
        name: 'Key 数',
        data: dbSeries.value.map((item) => item.value),
        barMaxWidth: 38,
        itemStyle: {
          borderRadius: [2, 2, 0, 0],
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: palette.barColorStart },
            { offset: 1, color: palette.barColorEnd },
          ]),
        },
      },
    ],
  };
};

const renderCharts = () => {
  if (opsChart) {
    opsChart.setOption(buildOpsOption(), true);
  }
  if (dbChart) {
    dbChart.setOption(buildDbOption(), true);
  }
};

const resetChartData = () => {
  opsData.value = [];
  opsLabels.value = [];
  dbSeries.value = [];
  opsChart?.clear();
  dbChart?.clear();
  renderCharts();
};

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
  }
  if (dbChartRef.value && !dbChart) {
    dbChart = echarts.init(dbChartRef.value);
  }
  renderCharts();
};

const poll = async () => {
  try {
    if (!connectionStore.activeConnection?.id) {
      resetChartData();
      return;
    }

    const infoRes = await getRedisServerInfo(connectionStore.activeConnection.id);
    if (!infoRes.success || !infoRes.data) {
      message.error(infoRes.message || "获取服务器信息失败");
      return;
    }
    Object.assign(serverInfo, infoRes.data);

    const ops = Number(serverInfo.ops_per_sec ?? 0);
    const now = new Date();
    const label = `${now.getHours()}:${String(now.getMinutes()).padStart(2, '0')}:${String(now.getSeconds()).padStart(2, '0')}`;
    opsLabels.value.push(label);
    opsData.value.push(ops);
    if (opsLabels.value.length > 30) {
      opsLabels.value.shift();
      opsData.value.shift();
    }

    const dbCountRes = await getDbCount(connectionStore.activeConnection.id);
    if (!dbCountRes.success || dbCountRes.data === undefined) {
      message.error(dbCountRes.message || "获取数据库数量失败");
      return;
    }
    const dbCount = dbCountRes.data;

    const series: { name: string; value: number }[] = [];
    for (let i = 0; i < dbCount; i++) {
      const countRes = await getDbKeyCount(connectionStore.activeConnection.id, i);
      if (countRes.success && countRes.data !== undefined) {
        series.push({ name: `db${i}`, value: countRes.data });
      }
    }
    dbSeries.value = series;

    renderCharts();
  } catch (e: any) {
    message.error(`获取统计信息失败: ${e}`);
  }
};

onMounted(() => {
  initCharts();
  timer = setInterval(poll, 3000);
  poll();
  disposeThemeListener = themeStore.themeChange(() => {
    renderCharts();
  });
  window.addEventListener('resize', resize);
});

onBeforeUnmount(() => {
  clearInterval(timer);
  window.removeEventListener('resize', resize);
  opsChart?.dispose();
  dbChart?.dispose();
  disposeThemeListener?.();
});

watch(
  () => connectionStore.activeConnectionId,
  (newId, oldId) => {
    if (newId !== oldId) {
      Object.keys(serverInfo).forEach((key) => delete serverInfo[key]);
      resetChartData();
      poll();
    }
  },
);

const resize = () => {
  opsChart?.resize();
  dbChart?.resize();
}
</script>

