use crate::infra::db::entities::{persons, prelude::Persons};
use gg_core::domain::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Debug, Clone)]
pub struct PersonRepository {
    connection: DatabaseConnection,
}

impl PersonRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl LoadPersonsPort for PersonRepository {
    async fn load_persons(&self, user_id: UserId) -> Vec<Person> {
        let persons = Persons::find()
            .filter(persons::Column::CreatorId.eq(user_id.value()))
            .all(&self.connection)
            .await;

        match persons {
            Ok(persons) => persons
                .into_iter()
                .map(|person| Person::new(
                    PersonId::new(person.uid),
                    UserId::new(person.creator_id),
                    PersonName::new(person.name),
                    PersonDescription::new(person.description),
                ))
                .collect(),
            Err(err) => {
                log::error!("Failed to load persons: {}", err);

                return vec![]
            },
        }
    }
}

impl SavePersonPort for PersonRepository {
    async fn save_person(&self, person: Person) -> DomainResult<Person, SavePersonPortError> {
        let new_person = persons::ActiveModel {
            uid: Set(person.uid().value().to_owned()),
            creator_id: Set(person.creator_id().value().to_owned()),
            name: Set(person.name().value().to_owned()),
            description: Set(person.description().value().to_owned()),
        };

        match new_person.insert(&self.connection).await {
            Ok(inserted_person) => Ok(Person::new(
                PersonId::new(inserted_person.uid),
                UserId::new(inserted_person.creator_id),
                PersonName::new(inserted_person.name),
                PersonDescription::new(inserted_person.description),
            )),
            Err(err) => Err(DomainError::ExternalServiceError(
                SavePersonPortError::InternalError(err.to_string()),
            )),
        }
    }
}
