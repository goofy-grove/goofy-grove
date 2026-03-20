<script setup lang="ts">
import { AppButton, AppInput, AppText } from '@/shared/ui';
import { useAuth } from './hooks';

const { username, password, error, authorize } = useAuth();
</script>

<template>
  <div class="auth-page">
    <div class="auth-form">
      <app-text type="error" v-if="error">{{ error }}</app-text>
      <app-input
        v-model="username"
        id="username"
        :label="$t('login.username')"
        :placeholder="$t('login.labels.username')"
        autocomplete="off"
      />
      <app-input
        v-model="password"
        id="password"
        :label="$t('login.password')"
        type="password"
        :placeholder="$t('login.labels.password')"
        autocomplete="off"
      />
      <app-button class="auth-form__submit-button" @click="authorize">
        {{ $t('login.login') }}
      </app-button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.auth-page {
  height: 300px;
  width: 400px;
  padding: 20px;
  border: 1px solid #434343;
  background-color: #191919;
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.25);
  border-radius: 12px;
  margin: auto;
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 10px;
  height: 100%;

  &__submit-button {
    margin-top: auto;
  }
}
</style>
