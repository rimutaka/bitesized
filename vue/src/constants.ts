/// The endpoint for question-related requests.
export const QUESTION_HANDLER_URL = "https://bitesized.info/q?";
/// The endpoint for user-related requests.
export const USER_HANDLER_URL = "https://bitesized.info/u?";

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
  { t: "General", id: "general" },
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

/// A mirror of the Rust's type
export interface Answer {
  /// Answer option
  a: string,
  /// Detailed explanation
  e: string,
  /// Set to true if correct
  c: boolean,
  /// Set to true if this is the user selection
  sel: boolean,
}

/// A mirror of the Rust's type
export interface Stats {
  correct: number,
  incorrect: number,
  skipped: number,
}

/// A mirror of the Rust's type
export interface Question {
  /// Leave blank for new questions
  qid: string,
  /// Required - use the TOPICS.id
  topic: string,
  question: string,
  answers: Array<Answer>,
  /// It is recalculated on the server on submission
  correct: number,
  /// A hash of the email address of the original author
  /// The value submitted by the user is ignored
  readonly author: string | undefined,
  /// When the question was last updated
  updated: string | undefined,
  /// Stats for how the question is used
  stats: Stats | undefined,
}

/// A mirror of the Rust's type
export interface User {
  /// User's email address
  email: String,
  /// The list of subscribed topics
  topics: string[],
  /// A unique string to use an unsubscribe key
  /// A shortened base58 encoded UUID
  unsubscribe: string,
  /// When the user sub was last updated
  updated: string,
}

/// Indicates the status of loading / fetching data
export enum LoadingStatus {
  Loading, // awaiting response
  Loaded, // data found and is loaded into the app
  NoData, // checked the DB, but no data found
  Error // something went wrong
}