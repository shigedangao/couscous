<script setup lang="ts">
import MainComponent from '../components/MainComponent.vue'
</script>

<template>
  <main>
    <Suspense>
      <MainComponent />
    </Suspense>
  </main>
</template>
