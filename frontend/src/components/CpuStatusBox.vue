<template>
<v-card class="w-80 h-100" :title="statusEmoji + ' ' + heading">

    <div class="ml-4 mt-2"><span class="text-6xl">{{ percentage.toFixed(2) }}</span> <span class="text-2xl font-bold">%</span></div>
    <div class="ml-5 font-bold text-2xl decoration-2">Usage</div>
    <v-sparkline
      color="rgb(var(--v-theme-primary))"
      fill
      :gradient="['color-mix(in srgb, currentColor 70%, transparent)', 'color-mix(in srgb, currentColor 15%, transparent)', 'transparent']"
      :height="250"
      :line-width="2"
      :model-value="percentages"
      :padding="12"
      :smooth="8"
      :width="500"
/>

<div v-if="total !== undefined && inUse !== undefined" class="ml-4 mt-2 flex gap-2 items-center justify-between mr-4">
    <div class="flex flex-col"><div>Average</div><div><span class="text-xl">{{ average }}%</span></div></div>
    <div class="flex flex-col"><div>High</div><div><span class="text-xl">{{  (inUse / 1024 / 1024 / 1024).toFixed(2) }} GB</span></div></div>
    <div class="flex flex-col"><div>Low</div><div><span class="text-xl">{{  ((total / 1024 / 1024 / 1024) - (inUse / 1024 / 1024 / 1024)).toFixed(2) }} GB</span></div></div>
</div>



  </v-card>



</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'

const props = defineProps<{ heading: string, percentage: number, total?: number, inUse?: number, percentageDescription?: string, totalDescription?: string, inUseDescription?: string }>()

const color = computed(() => {
    if (props.percentage > 80) return 'red'
    if (props.percentage > 50) return 'yellow'
    return 'green'
})

const percentages = ref<number[]>([])
const average = computed(() => {
    if (percentages.value.length === 0) return 0
    return percentages.value.reduce((sum, val) => sum + val, 0) / percentages.value.length
})
const statusEmoji = computed(() => {
    if (props.percentage > 50) return '⚠️'
    return '✅'
})


watch(() => props.percentage, () => {
    percentages.value.push(props.percentage)
})
</script>
