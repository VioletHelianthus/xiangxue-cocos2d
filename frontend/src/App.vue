<template>
  <PreviewShell v-if="config" :config="config">
    <ComplexRPG />
  </PreviewShell>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { PreviewShell, loadConfig } from '@xiangxue/preview'
import type { PreviewConfig } from '@xiangxue/preview'
import ComplexRPG from './pages/ComplexRPG.vue'

const config = ref<PreviewConfig | null>(null)

onMounted(async () => {
  config.value = await loadConfig()
})
</script>
