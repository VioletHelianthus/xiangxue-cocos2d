<template>
  <div
    data-widget="ScrollView"
    :data-name="name"
    :data-direction="direction"
    :data-bounce="bounce ? 'true' : undefined"
    :data-inner-width="innerWidth"
    :data-inner-height="innerHeight"
    :style="rootStyle"
    class="cocos-scrollview"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  direction?: 'vertical' | 'horizontal' | 'both'
  bounce?: boolean
  innerWidth?: string
  innerHeight?: string
  width?: string
  height?: string
}>(), {
  direction: 'vertical',
  bounce: true,
  width: '300px',
  height: '400px',
})

const rootStyle = computed(() => ({
  width: props.width,
  height: props.height,
  overflow: 'auto',
  position: 'relative' as const,
  minWidth: props.innerWidth || undefined,
  minHeight: props.innerHeight || undefined,
}))
</script>

<style scoped>
.cocos-scrollview {
  border: 1px solid #444;
  background: #1a1a1a;
}
</style>
