/// The endpoint for question-related requests.
export const QUESTION_HANDLER_URL = "https://bitesized.info/q?";
/// The endpoint for user-related requests.
export const USER_HANDLER_URL = "https://bitesized.info/u?";
/// The endpoint for payment-related requests.
export const PAYMENTS_HANDLER_URL = "https://bitesized.info/checkout?";

/// E.g. .../q?topic=foo&qid=bar
export const URL_PARAM_TOPIC = "topic"
/// E.g. .../q?topics=foo,bar
export const URL_PARAM_TOPICS = "topics"
/// E.g. .../q?topic=foo&qid=bar
export const URL_PARAM_QID = "qid"
/// E.g. .../q?topic=foo&qid=bar&answers=0.1
export const URL_PARAM_ANSWERS = "answers"
/// A character used to separate values within the same param value,
/// e.g. .../q?topics=foo.bar
export const URL_PARAM_LIST_SEPARATOR = "."

/// A temporary measure to limit who can save data in DDB
export const TOKEN_HEADER_NAME = "x-bitie-token";
/// A comma-separated list of recently viewed questions
export const RECENT_HEADER_NAME = "x-bitie-recent";

/// The key name for the last authentication timestamp in the localStorage.
/// The user is asked to auth if the key is present.
export const LAST_AUTH_TIMESTAMP = "auth";

/// Used for defining emits, e.g. defineEmits([VUE_EVENT_HYDRATED]);
export const VUE_EVENT_HYDRATED = "hydrated";

/// The name of the preview question in the localStorage and the name of the popup window
/// with live preview rendering
export const PREVIEW_QUESTION_LS_KEY = "previewQuestion";

/// A locally cached value of the last contributor details entered by this user
/// for future reuse with new questions
export const CONTRIBUTOR_DETAILS_LS_KEY = "contributor";

/// A locally cached value of the last sponsor details entered by this user
/// for future reuse with new payments
export const SPONSOR_DETAILS_LS_KEY = "sponsorship";

export const MAX_NUMBER_OF_QUESTIONS_PER_PAYMENT = 20;

/// Keypair for the topic title and DDB topic ID, e.g. "AWS"/"aws".
export interface TopicFields {
  t: string,
  id: string,
};

/// The exclusive list of topics that can be used in the app
/// It is sync'd with the DDB and the lambdas manually
export const TOPICS = <Array<TopicFields>>[
  { t: "AWS", id: "aws" },
  { t: "CSS", id: "css" },
  { t: "Essentials", id: "general" },
  { t: "JS / TS", id: "js-ts" },
  { t: "Rust", id: "rust" }
];

/// A special topic that shows questions from all topics
/// this is a temporary hack until the randomness madness is sorted out
export const ANY_TOPIC = "any";

/// Returns the topic title by its ID
export function findTopicById(id: string | undefined): string | undefined {
  if (!id) return undefined;
  return TOPICS.find((topic) => topic.id === id)?.t;
}
