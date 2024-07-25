use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub mod expense {
    use crate::m20220101_000002_create_table_budget::budgets;
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;
    use sea_orm::entity::prelude::*;
    use sea_orm_migration::sea_orm;
    use uuid::Uuid;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "expense")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub budget_id: Uuid,
        pub amount: Decimal,
        pub description: String,
        pub date: Date,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        Budget,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::Budget => Entity::belongs_to(budgets::Entity)
                    .from(Column::BudgetId)
                    .to(budgets::Column::Id)
                    .into(),
            }
        }
    }

    impl Related<budgets::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Budget.def()
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
            .create_table(schema.create_table_from_entity(expense::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(expense::Entity).to_owned())
            .await
    }
}
