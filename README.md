# Bite-Sized Learning for Busy Software Engineers

**This project is work in progress, extremely unstable and should not be used for anything.**

## Overview

_Bite-Sized_ is an open-source system for testing, refreshing and reinforcing knowledge in 2 minutes or less.
- it emails you a question with a number of possible answers on a topic of your choosing
- you choose the right answer and get to see the explanations behind different options

### Sample question

```
A large DynamoDB table has the following structure:
- `UserID` (PK)
- `ItemID` (SK)
- `ItemBarcode`
- other fields

What is the most efficient way of fetching a list of users (`UserID`) by `ItemBarcode`?

1. Perform a table Scan with `ItemBarcode` as the FilterExpression
2. Create a Local Secondary Index for `ItemBarcode` and perform Query with `ItemBarcode` as the key
3. Create a Global Secondary Index for `ItemBarcode` and perform Query with `ItemBarcode` as the only key
```

The correct answer:
```
A Global Secondary Index (GSI) is like a replica of the main table with different _Partition_ and _Secondary_ keys.
GSI items include the table Partition Key by default. All or some other attributes can be added as needed.
GSI's incur additional RCUs for maintenance.
https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.html
```

## Features

Technical features:
- subscribes learners to topics
- sends questions with a selection of answers via email on a schedule
- displays the emailed question on a web page
- accepts answers as HTTP calls from email bodies
- displays detailed answers in response to the answer call
- one-click unsubscribe

## Deployment

### Dev env

* Front-end: `npm run build` + `git push` to let [.github/workflows/deploy.yml] copy the built files to the S3 bucket.

## Management

### Adding, editing and removing questions

### Managing learner accounts

### Viewing stats

## Tech stack and attribution

* Cheese logo nicked from https://www.freepik.com/icon/cheese_4900716
* Front-end: Vue
* CSS: https://v2.tailwindcss.com/docs/
* UI components: https://primevue.org/
* 