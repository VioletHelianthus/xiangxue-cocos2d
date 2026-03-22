<template>
  <div
    data-widget="TextField"
    :data-name="name"
    :data-max-length="maxLength ? String(maxLength) : undefined"
    :data-password="password ? 'true' : undefined"
    :placeholder="placeholder"
    :style="rootStyle"
    class="cocos-textfield"
  >
    <span v-if="text" class="cocos-textfield__text">{{ displayText }}</span>
    <span v-else class="cocos-textfield__placeholder">{{ placeholder }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  text?: string
  placeholder?: string
  maxLength?: number
  password?: boolean
  width?: string
  height?: string
  fontSize?: string
}>(), {
  text: '',
  placeholder: '',
  password: false,
  width: '200px',
  height: '36px',
  fontSize: '16px',
})

const displayText = computed(() =>
  props.password ? '•'.repeat(props.text.length) : props.text
)

const rootStyle = computed(() => ({
  width: props.width,
  height: props.height,
  display: 'flex',
  alignItems: 'center',
  padding: '0 8px',
  fontSize: props.fontSize,
}))
</script>

<style scoped>
.cocos-textfield {
  border: 1px solid #666;
  border-radius: 3px;
  background: #2a2a2a;
  color: #eee;
  box-sizing: border-box;
}
.cocos-textfield__placeholder {
  color: #888;
}
</style>
