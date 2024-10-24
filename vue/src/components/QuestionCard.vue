<template>
  <TransitionSlot>
    <div class="flex" v-if="questionMarkdown">
      <div class="q-card">

        <div class="q-text">
          <div class="" v-html="questionMarkdown?.question"></div>
        </div>

        <div v-for="(answer, index) in questionMarkdown?.answers" :key="index">
          <h3 v-if="isAnswered && index === 0 && questionMarkdown?.correct == 1" class="mb-4">Your answer</h3>
          <h3 v-if="isAnswered && index === 0 && questionMarkdown?.correct > 1" class="mb-4">Your answers</h3>
          <h3 v-else-if="!isAnswered && index === 0" class="mb-4">Answers</h3>
          <h3 v-else-if="isAnswered && index === questionMarkdown?.correct" class="mb-4">Other options</h3>

          <div class="mb-8 border-2 rounded-md" :class="{ 'border-green-100': answer?.c, 'border-red-100': !answer?.c && isAnswered, 'border-slate-100': !isAnswered }">
            <div class="flex items-center" :class="{ 'bg-green-100': answer?.c, 'bg-red-100': !answer?.c && isAnswered }">

              <input type="radio" v-if="questionMarkdown?.correct == 1" :name="questionMarkdown?.qid" :value="index" :disabled="isAnswered" v-model="learnerAnswerRadio" />
              <input type="checkbox" v-if="questionMarkdown?.correct && questionMarkdown.correct > 1" :name="questionMarkdown?.qid" :disabled="isAnswered" :value="index" v-model="learnerAnswersCheck" />
              <div class="q-answer" v-html="answer.a"></div>

            </div>

            <div v-if="answer?.c" class="px-2">Correct.</div>
            <div v-else-if="isAnswered" class="px-2">Incorrect.</div>
            <div class="q-explain" v-if="answer?.e" v-html="answer.e"></div>
          </div>
        </div>

        <div class="flex">
          <div v-if="hasToken" class="flex-shrink text-start">
            <Button label="Edit" icon="pi pi-pencil" severity="secondary" rounded class="whitespace-nowrap" @click="navigateToEditPage" />
          </div>
          <div v-if="!isAnswered" class="flex-grow text-end my-auto me-4">
            <p class="text-xs text-slate-500">{{ optionsToSelect }}</p>
          </div>
          <div v-if="!isAnswered" class="flex-shrink text-end">
            <Button label="Submit" icon="pi pi-check" raised rounded class="font-bold px-24 py-4 my-auto whitespace-nowrap" :disabled="!isQuestionReady" @click="submitQuestion()" />
          </div>
        </div>
      </div>
    </div>
  </TransitionSlot>
  <div v-if="!questionMarkdown">
    <p>Loading...</p>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, watch } from "vue";
import type { Question } from "@/constants";
import { QUESTION_HANDLER_URL, URL_PARAM_QID, URL_PARAM_TOPIC, TOKEN_HEADER_NAME } from "@/constants";
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";
import Button from 'primevue/button';
import TransitionSlot from "./TransitionSlot.vue";
import router from "@/router";

const props = defineProps<{
  topic: string,
  qid?: string,
}>()

// as fetched from the server
const questionMarkdown = ref<Question | undefined>();
const learnerAnswersCheck = ref<string[]>([]);
const learnerAnswerRadio = ref<string | undefined>();

// a temporary solution to enable editing links
const hasToken = computed(() => {
  return localStorage.getItem(TOKEN_HEADER_NAME) ? true : false;
});

const isAnswered = computed(() => {
  if (questionMarkdown.value?.answers?.[0].e) { return true } else { return false };
});

const isQuestionReady = computed(() => {
  // console.log("isQuestionReady", learnerAnswerRadio.value, learnerAnswersCheck.value, questionMarkdown.value?.correct);
  // must be either a single radio answer or multiple checkbox answers matching the number of correct answers
  return learnerAnswerRadio.value !== undefined && questionMarkdown.value?.correct == 1 || learnerAnswersCheck.value.length == questionMarkdown.value?.correct;
});

// this fn abstracts the complicated logic of informing the user
// how many answers they need to select
const optionsToSelect = computed(() => {
  // this line is needed for type checking
  if (!questionMarkdown.value) return "";

  // is this a single choice question with radio buttons?
  if (questionMarkdown.value.correct == 1) {
    // if a radio button is selected it can be submitted
    // the user can change the answer, but not deselect it
    if (learnerAnswerRadio.value == undefined) {
      return "Select one of the options";
    } else {
      return "Check your selection and submit";
    }
  }

  // assume it is a multichoice question from now on

  // perform an exhaustive match by how many answers were selected
  const remainingNumber = questionMarkdown.value.correct - learnerAnswersCheck.value.length;
  if (remainingNumber == 0) {
    // if the required number of answers is selected, the user can submit
    return "Check your selection and submit";
  }
  else if (remainingNumber < 0) {
    // too many answers selected
    return `Only ${questionMarkdown.value.correct} options should be selected`;
  }
  else {
    // more answers should be selected
    const wordMore = learnerAnswersCheck.value.length ? "more" : "";
    const wordAnswers = remainingNumber > 1 ? "options" : "option";
    return `Select ${remainingNumber} ${wordMore} ${wordAnswers}`;
  }
});

async function submitQuestion() {
  // double-check there are answers to submit
  if (!isQuestionReady.value) {
    console.error("Must select answers:", questionMarkdown.value?.correct);
    return;
  }

  // the lambda expects a string array of answers
  const answers = JSON.stringify(questionMarkdown.value?.correct == 1 ? [learnerAnswerRadio.value] : learnerAnswersCheck.value);

  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  const hash = new Sha256();
  hash.update(answers);
  const bodyHash = toHex(await hash.digest());

  const response = await fetch(`${QUESTION_HANDLER_URL}${URL_PARAM_TOPIC}=${questionMarkdown.value?.topic}&${URL_PARAM_QID}=${questionMarkdown.value?.qid}`, {
    method: "POST",
    body: answers,
    headers: {
      "x-amz-content-sha256": bodyHash,
    },
  });

  // a successful response should contain the saved question
  // an error may contain JSON or plain text, depending on where the errror occurred
  if (response.status === 200) {
    try {
      // update the question with the full details
      const question = <Question>await response.json();
      questionMarkdown.value = question;
      console.log("Full question received", questionMarkdown.value);

      // reset the user selection because the answers got rearranged with the correct ones at the top
      learnerAnswersCheck.value = [];
      question.answers.forEach((answer, index) => {
        if (answer.sel) {
          if (question.correct == 1) {
            learnerAnswerRadio.value = index.toString();
            console.log("learnerAnswerRadio", learnerAnswerRadio.value);
          } else {
            learnerAnswersCheck.value.push(index.toString());
            console.log("learnerAnswersCheck", learnerAnswersCheck.value);
          }
        }
      });

    } catch (error) {
      console.error(error);
    }
  } else {
    console.error("Failed to save the question: ", response.status);
  }
}

/// navigates to edit page, but it should only work if the user has a token
function navigateToEditPage() {
  router.push(`/add?${URL_PARAM_TOPIC}=${questionMarkdown.value?.topic}&${URL_PARAM_QID}=${questionMarkdown.value?.qid}`);
}

watchEffect(async () => {
  console.log("fetching question for topic", props.topic);
  // only fetch if topic is set
  if (!props.topic) return;

  questionMarkdown.value = undefined;

  try {

    // fetching by topic returns a random question
    // fetching with qid returns a specific question
    const fetchParams = `${URL_PARAM_TOPIC}=${props.topic}`.concat(props.qid ? `&${URL_PARAM_QID}=${props.qid}` : "");
    console.log("fetchParams", fetchParams);

    const response = await fetch(`${QUESTION_HANDLER_URL}${fetchParams}`);
    console.log(`Fetched. Status: ${response.status}`);

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        const question = <Question>await response.json();
        console.log(question);
        // console.log(question.topic);
        // console.log(question.qid);

        questionMarkdown.value = question;

        // useRouter().push(`/question/${savedQuestion.topic}/${savedQuestion.qid}`);
      } catch (error) {
        console.error(error);
      }
    } else {
      console.error("Failed to get question. Status: ", response.status);
    }
  } catch (error) {
    console.error("Failed to get question.");
    console.error(error);
  }
});

</script>
