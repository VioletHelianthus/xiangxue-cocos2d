<template>
  <!-- SkillPanel.csd: 技能面板，ListView 里放一个 ProjectNode 引用 SkillItem.csd -->
  <div data-name="skillPanel" style="width:616px;height:300px;display:flex;flex-direction:column;gap:6px;padding-left:12px;padding-right:12px;padding-top:8px">
    <span data-name="title" style="height:22px;color:#ddd">Skills</span>
    <CocosListView name="skillList" direction="vertical" :item-margin="8" width="592px" height="260px">
      <CocosProjectNode name="itemTemplate" file="SkillItem.csd" width="580px" height="64px" />
    </CocosListView>
  </div>
</template>

<script setup lang="ts">
</script>
