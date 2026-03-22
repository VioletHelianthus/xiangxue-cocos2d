<template>
  <!-- 独立 item 组件 → 独立 SkillItem.csd -->
  <div data-name="skillItem" style="width:580px;height:64px;display:flex;flex-direction:row;align-items:center;gap:10px;padding-left:6px;padding-right:6px">
    <img data-name="icon" :src="icon" style="width:48px;height:48px" />
    <div data-name="info" style="display:flex;flex-direction:column;gap:2px">
      <span data-name="skillName" style="width:200px;height:20px">{{ name }}</span>
      <span data-name="skillDesc" style="width:300px;height:16px;color:#999">{{ desc }}</span>
    </div>
    <div style="flex-grow:1"></div>
    <span data-name="cost" style="width:60px;height:18px;color:#88f">{{ costText }}</span>
  </div>
</template>

<script setup lang="ts">
// Props 仅供预览/示例用，实际数据由代码端填充
withDefaults(defineProps<{
  name?: string
  desc?: string
  icon?: string
  costText?: string
}>(), {
  name: 'Skill Name',
  desc: 'Skill description',
  icon: '/ui-pack-rpg/PNG/iconCircle_brown.png',
  costText: 'MP 0',
})
</script>
