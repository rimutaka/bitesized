<template>
  <div class="w-full mb-4 flex gap-8 justify-center border-t pt-2 mt-12">
      <template v-for="item in menuItems" :key="item.label">
        <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
          <a v-ripple :href="href" @click="navigate">
            <span :class="item.icon" />
            <span class="ml-2">{{ item.label }}</span>
          </a>
        </router-link>
        <a v-else v-ripple :href="item.url">
          <span :class="item.icon" />
          <span class="ml-2">{{ item.label }}</span>
        </a>
      </template>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";


const menuItems = ref([
  {
    label: 'Home',
    icon: 'pi pi-home',
    route: '/'
  },
  {
    label: 'About',
    icon: 'pi pi-star',
    route: '/about'
  },
  {
    label: 'Contact',
    icon: 'pi pi-envelope',
    badge: 3,
    url: 'mailto:max@onebro.me'
  }
]);
</script>