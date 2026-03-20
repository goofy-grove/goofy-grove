<script setup lang="ts">
import { getAvailableLocales, loadLocale } from '@/shared/libs';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';

const availableLocales = ref<Record<string, string>>({});
const i18n = useI18n();
const currentLocale = computed({
  get: () => i18n.locale.value,
  set: async (value) => {
    await loadLocale(value);
    i18n.locale.value = value;
  },
});

getAvailableLocales().then(({ locales }) => (availableLocales.value = locales));
</script>

<template>
  <div class="app-language-switcher">
    <select class="app-language-switcher__select" v-model="currentLocale">
      <option
        v-for="([code, label]) in Object.entries(availableLocales)"
        :key="code"
        :value="code"
      >
        {{ label }}
      </option>
    </select>
  </div>
</template>

<style lang="scss" scoped>
.app-language-switcher {
  position: absolute;
  right: 10px;
  top: 10px;

  &__select {
    padding: 10px 20px;
    border: none;
    color: var(--text-color);
    background-color: #333;
    border-radius: 6px;
    cursor: pointer;
    font-size: 1.2rem;
  }
}
</style>
