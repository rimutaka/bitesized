<template>
  <div class="card mt-4">
    <h3>Select your topics of interest</h3>
    <TopicsList />

    <div class="flex flex-wrap mt-12 mb-4">
      <div class="flex-shrink text-start mx-auto">
        <Button :label="question ? 'Browse more questions or subscribe' : 'Browse questions or subscribe'" icon="pi pi-envelope" raised rounded class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="navigateToSubscription" />
        <p class="text-xs text-center md:mb-auto text-slate-500 dark:text-slate-200">You will be asked to
          <router-link :to="{ name: PageIDs.SUBSCRIPTION }">sign in</router-link>
          with <i class="pi pi-github ms-1 me-2"></i><i class="pi pi-google me-2"></i>
        </p>
      </div>
      <p class="w-full text-center my-4">or</p>
      <div class="flex-grow text-center mb-4">
        <RandomQuestionButton />
      </div>
    </div>
    <TransitionSlot>
      <SampleQuestion v-if="showingSampleQuestion" />
    </TransitionSlot>
  </div>
</template>


<script setup lang="ts">
import { ref } from 'vue';
import router from '@/router';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { TOPICS, URL_PARAM_LIST_SEPARATOR } from "@/constants";
import { PageIDs } from '@/router';

import Button from 'primevue/button';
import TopicsList from './TopicsList.vue';
import TransitionSlot from "./TransitionSlot.vue";
import SampleQuestion from "./SampleQuestion.vue";
import RandomQuestionButton from './RandomQuestionButton.vue';

const store = useMainStore();
const { selectedTopics, question, showingSampleQuestion } = storeToRefs(store);

async function navigateToSubscription() {
  console.log("Subscribing to topics: ", selectedTopics.value);
  const subTopics = selectedTopics.value.map((t) => t).join(URL_PARAM_LIST_SEPARATOR);
  if (subTopics) {
    router.push({ name: 'subscription', query: { topics: subTopics } });
  } else {
    router.push({ name: 'subscription' });
  }
}

// do not show the sample question when the page is first displayed
// it will be shown when the user clicks on the button requesting it
showingSampleQuestion.value= false;

</script>
