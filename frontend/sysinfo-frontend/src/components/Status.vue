<template>
<v-app-bar>
  <template v-slot:prepend>
    <v-app-bar-nav-icon></v-app-bar-nav-icon>
  </template>

  <v-app-bar-title>System Status</v-app-bar-title>
</v-app-bar>

<v-container>
    <h1 class="text-2xl">Hardware Information</h1>
    <v-container >
    <cpu-status-box v-if="status" description="Usage" heading="CPU" :percentage="status.cpu_info.percentage.toFixed(2)" />

    <memory-status-box
v-if="status"
description="Usage"
heading="Memory"
:in-use="status.memory_info.used_memory"
in-use-description="In Use"
:percentage="(status.memory_info.used_memory / status.memory_info.total_memory * 100).toFixed(2)"
percentage-description="Usage"
:total="status.memory_info.total_memory"
total-description="Total Memory"
/>
  </v-container>
  <v-divider></v-divider>
</v-container>
</template>

<script setup lang="ts">
import { connectWebSocket } from '@/api/statuscheck'
import CpuStatusBox from '@/components/CpuStatusBox.vue'
import MemoryStatusBox from '@/components/MemoryStatusBox.vue'

const status = connectWebSocket()

</script>

<style scoped>
@reference "../styles/tailwind.css";
</style>
