use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub mod budgets {
    use crate::m20220101_000001_create_table_user::user;
    use crate::m20220101_000003_create_table_expense::expense;
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;
    use sea_orm::entity::prelude::*;
    use sea_orm_migration::sea_orm;
    use uuid::Uuid;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "budget")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub user_id: Uuid,
        pub name: String,
        pub total_amount: Decimal,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        User,
        Expense,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::User => Entity::belongs_to(user::Entity)
                    .from(Column::UserId)
                    .to(user::Column::Id)
                    .into(),
                Self::Expense => Entity::has_many(expense::Entity).into(),
            }
        }
    }

    impl Related<user::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }

    impl Related<expense::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Expense.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        manager
            .create_table(schema.create_table_from_entity(budgets::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(budgets::Entity).to_owned())
            .await
    }
}
