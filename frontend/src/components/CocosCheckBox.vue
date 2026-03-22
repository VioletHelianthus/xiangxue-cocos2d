<template>
  <div
    data-widget="CheckBox"
    :data-name="name"
    :data-bg-normal="bgNormal"
    :data-bg-pressed="bgPressed"
    :data-bg-disabled="bgDisabled"
    :data-cross-normal="crossNormal"
    :data-cross-disabled="crossDisabled"
    :style="rootStyle"
    class="cocos-checkbox"
  >
    <div class="cocos-checkbox__box" :class="{ checked }">
      <span v-if="checked" class="cocos-checkbox__mark">✓</span>
    </div>
    <span v-if="$slots.default" class="cocos-checkbox__label"><slot /></span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  checked?: boolean
  bgNormal?: string
  bgPressed?: string
  bgDisabled?: string
  crossNormal?: string
  crossDisabled?: string
  width?: string
  height?: string
}>(), {
  checked: false,
  width: '40px',
  height: '40px',
})

const rootStyle = computed(() => ({
  display: 'inline-flex',
  alignItems: 'center',
  gap: '8px',
  width: props.width,
  height: props.height,
}))
</script>

<style scoped>
.cocos-checkbox__box {
  width: 24px;
  height: 24px;
  border: 2px solid #888;
  border-radius: 3px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #333;
  flex-shrink: 0;
}
.cocos-checkbox__box.checked {
  background: #4a9;
  border-color: #4a9;
}
.cocos-checkbox__mark {
  color: #fff;
  font-size: 16px;
  font-weight: bold;
}
.cocos-checkbox__label {
  color: #eee;
  font-size: 14px;
}
</style>
