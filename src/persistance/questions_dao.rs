use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(
            r#"
                INSERT INTO questions ( title, description )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            question.title,
            question.description
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e: sqlx::Error| DBError::Other(Box::new(e)))?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse answer UUID: {}", question_uuid))
        })?;

        sqlx::query!(
            r#"
                DELETE FROM questions WHERE question_uuid = $1
            "#,
            uuid,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e: sqlx::Error| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records = sqlx::query!(
            r#"
                SELECT * FROM questions
            "#,
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e: sqlx::Error| DBError::Other(Box::new(e)))?;

        // Iterate over `records` and map each record to a `QuestionDetail` type
        let questions = records
            .into()
            .map(|r| QuestionDetail {
                question_uuid: r.question_uuid.to_string(),
                title: r.title,
                description: r.description,
                created_at: r.created_at.to_string(),
            })
            .collect();

        Ok(questions)
    }
}
