<template>
   <i-frame v-model:src="url"></i-frame>
</template>

<script setup>
import iFrame from '@/components/iFrame'
import {ref} from "vue";

const url = ref(import.meta.env.VITE_APP_BASE_API + "/swagger-ui")
</script>
