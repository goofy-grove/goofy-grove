<script setup lang="ts"></script>

<template>
  <button class="app-button"><slot /></button>
</template>

<style scoped lang="scss">
.app-button {
  padding: 10px 20px;
  border: none;
  color: var(--text-color);
  background-color: #333;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1.2rem;
}

.app-button:hover {
  background-color: #434343;
}
</style>
