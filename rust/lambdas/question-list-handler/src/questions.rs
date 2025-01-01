use aws_sdk_dynamodb::{types::AttributeValue, Client as DdbClient};
use bitie_types::{ddb::fields, ddb::tables, question::Question, topic::Topic};
use chrono::{DateTime, Utc};
use tracing::{error, info, warn};

/// Returns True if the param a single valid topic,
/// otherwise returns false.
pub(crate) fn validate_topic(topic: &str) -> bool {
    !topic.is_empty() && Topic::filter_valid_topics(vec![topic.to_string()]).len() == 1
}

/// Returns a list questions for the given topic.
/// Not all Question fields are included because this query uses an index.
/// Returns a error if no questions found.
pub(crate) async fn get_all_questions_by_topic(client: &DdbClient, topic: &str) -> Option<Vec<Question>> {
    info!("Getting all questions for {topic}");
    // list of questions fetched from DDB
    let mut fetched_questions = Vec::new();

    // try to get the questions from DDB
    match client
        .query()
        .table_name(tables::QUESTIONS)
        .index_name(tables::QUESTIONS_IDX_TOPIC_QID)
        .key_condition_expression("#topic = :topic")
        .expression_attribute_names("#topic", fields::TOPIC)
        .expression_attribute_values(":topic", AttributeValue::S(topic.to_owned()))
        .send()
        .await
    {
        Ok(v) => {
            match v.items {
                Some(items) => {
                    // get all the items from the response
                    // TODO: add pagination when there are enough questions to fill multiple pages
                    for item in items.into_iter() {
                        let item_qid = match item.get(fields::QID) {
                            Some(AttributeValue::S(v)) => v.clone(),
                            _ => {
                                warn!("Invalid question for {topic}: missing qid attribute");
                                continue;
                            }
                        };

                        // get question title from an included attribute
                        let title = match item.get(fields::TITLE) {
                            Some(AttributeValue::S(v)) => Some(v.clone()),
                            _ => {
                                warn!("Invalid `title` attribute for {topic} / {item_qid}");
                                Some(Question::DEFAULT_TITLE.to_string())
                            }
                        };

                        // get updated field from an included attribute
                        let updated = match item.get(fields::UPDATED) {
                            Some(AttributeValue::S(v)) => match DateTime::parse_from_rfc3339(v) {
                                Ok(v) => Some(v.with_timezone(&Utc)),
                                Err(e) => {
                                    warn!("Invalid `updated` attribute for {topic} / {item_qid}: {:?}", e);
                                    Some(DateTime::<Utc>::MIN_UTC)
                                }
                            },
                            _ => {
                                warn!("Invalid `updated` attribute for {topic} / {item_qid}");
                                Some(DateTime::<Utc>::MIN_UTC)
                            }
                        };

                        let question = Question {
                            topic: topic.to_string(),
                            qid: item_qid,
                            title,
                            updated,
                            answers: Vec::new(),
                            question: "".to_string(),
                            correct: 0,
                            author: None,
                            contributor: None,
                            stats: None,
                        };

                        fetched_questions.push(question);
                    }
                }
                None => {
                    warn!("No query response for {topic}");
                }
            }
        }
        Err(e) => {
            error!("Query for {topic} failed: {:?}", e);
            return None;
        }
    }

    info!("Fetched questions: {}", fetched_questions.len());

    // sort the questions by updated date
    fetched_questions.sort_by(|a, b| b.updated.cmp(&a.updated));

    Some(fetched_questions)
}

/// Returns a list questions for the given author.
/// Not all Question fields are included because this query uses an index.
/// Returns None on error.
pub(crate) async fn get_all_questions_by_author(client: &DdbClient, email_hash: &str) -> Option<Vec<Question>> {
    info!("Getting author questions for {email_hash}");
    // list of questions fetched from DDB
    let mut fetched_questions = Vec::new();

    // try to get the questions from DDB
    match client
        .query()
        .table_name(tables::QUESTIONS)
        .index_name(tables::QUESTIONS_IDX_AUTHOR)
        .key_condition_expression("#author = :author")
        .expression_attribute_names("#author", fields::AUTHOR)
        .expression_attribute_values(":author", AttributeValue::S(email_hash.to_owned()))
        .send()
        .await
    {
        Ok(v) => {
            match v.items {
                Some(items) => {
                    // TODO: add pagination when there are enough questions to fill multiple pages
                    for item in items.into_iter() {
                        let topic = match item.get(fields::TOPIC) {
                            Some(AttributeValue::S(v)) => v.clone(),
                            _ => {
                                warn!("Invalid question for {email_hash}: missing topic attribute");
                                continue;
                            }
                        };

                        let qid = match item.get(fields::QID) {
                            Some(AttributeValue::S(v)) => v.clone(),
                            _ => {
                                warn!("Invalid question for {email_hash}: missing qid attribute");
                                continue;
                            }
                        };

                        // get question title from an included attribute
                        let title = match item.get(fields::TITLE) {
                            Some(AttributeValue::S(v)) => Some(v.clone()),
                            _ => {
                                warn!("Invalid `title` attribute for {topic} / {qid}");
                                Some(Question::DEFAULT_TITLE.to_string())
                            }
                        };

                        // get updated field from an included attribute
                        let updated = match item.get(fields::UPDATED) {
                            Some(AttributeValue::S(v)) => match DateTime::parse_from_rfc3339(v) {
                                Ok(v) => Some(v.with_timezone(&Utc)),
                                Err(e) => {
                                    warn!("Invalid `updated` attribute for {topic} / {qid}: {:?}", e);
                                    Some(DateTime::<Utc>::MIN_UTC)
                                }
                            },
                            _ => {
                                warn!("Invalid `updated` attribute for {topic} / {qid}");
                                Some(DateTime::<Utc>::MIN_UTC)
                            }
                        };

                        let question = Question {
                            topic,
                            qid,
                            title,
                            updated,
                            answers: Vec::new(),
                            question: "".to_string(),
                            correct: 0,
                            author: None,
                            contributor: None,
                            stats: None,
                        };

                        fetched_questions.push(question);
                    }
                }
                None => {
                    warn!("No query response for {email_hash}");
                }
            }
        }
        Err(e) => {
            error!("Query for {email_hash} failed: {:?}", e);
            return None;
        }
    }

    info!("Fetched questions: {}", fetched_questions.len());

    // sort the questions by updated date
    fetched_questions.sort_by(|a, b| b.updated.cmp(&a.updated));

    Some(fetched_questions)
}
