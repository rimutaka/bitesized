import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { TOPICS } from './constants'
import type { Question } from './constants'

/// The main store for the application
export const useMainStore = defineStore('main', () => {
  /// Currently loaded question
  const question = ref<Question | undefined>()

  /// All topics selected by the user
  const selectedTopics = ref<string[]>([])

  /// Either a random topic or the last selected topic
  /// The component decides which one to use
  const currentTopic = ref<string | undefined>()

  /// Email from the token from the ID provider, e.g. Google or Github
  const email = ref<string | undefined>()

  /// Raw token from the ID provider, e.g. Google or Github
  const token = ref<string | undefined>()

  /// Returns the last selected topic or a random one if none are selected
  const lastSelectedTopic = computed(() => {
    if (selectedTopics.value.length) {
      return selectedTopics.value[selectedTopics.value.length - 1];
    } else {
      return undefined;
    }
  });

  return { question, selectedTopics, lastSelectedTopic, currentTopic, email, token }
})