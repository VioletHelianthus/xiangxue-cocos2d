<template>
  <div
    data-widget="Sprite"
    :data-name="name"
    :src="texture"
    :style="rootStyle"
    class="cocos-sprite"
  >
    <img v-if="texture" :src="texture" class="cocos-sprite__img" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  texture: string
  width?: string
  height?: string
}>(), {
  width: '64px',
  height: '64px',
})

const rootStyle = computed(() => ({
  position: 'relative' as const,
  width: props.width,
  height: props.height,
}))
</script>

<style scoped>
.cocos-sprite__img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}
</style>
