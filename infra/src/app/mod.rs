use crate::repositories::{
    create_budget_repository::CreateBudgetRepository,
    get_one_budget_repository::GetOneBudgetRepository,
    sqlite_connection_pool::SqliteConnectionPool, update_budget_repository::UpdateBudgetRepository,
    CreateCategoryRepository, CreateUserRepository, GetAllBudgetsRepository,
    GetAllCategoriesRepository, GetAllUsersRepository, GetOneCategoryRepository,
    GetOneUserRepository, UpdateCategoryRepository, UpdateUserRepository,
};
use core::{
    app::{
        contracts::{
            CreateBudgetCommandHandler, CreateCategoryCommandHandler, CreateUserCommandHandler,
            GetAllBudgetsQueryHandler, GetAllCategoriesQueryHandler, GetAllUsersQueryHandler,
            GetOneBudgetQueryHandler, GetOneCategoryQueryHandler, GetOneUserQueryHandler,
            UpdateBudgetCommandHandler, UpdateCategoryCommandHandler, UpdateUserCommandHandler,
        },
        services::{
            CreateBudgetService, CreateCategoryService, CreateUserService, GetAllBudgetsService,
            GetAllCategoriesService, GetAllUsersService, GetOneBudgetService,
            GetOneCategoryService, GetOneUserService, UpdateBudgetService, UpdateCategoryService,
            UpdateUserService,
        },
    },
    domain::{
        errors::{GetAllBudgetsError, GetOneBudgetError},
        models::{
            CreateBudgetCommand, CreateCategoryCommand, CreateUserCommand, GetAllBudgetsQuery,
            GetAllBudgetsQueryResult, GetAllCategoriesQuery, GetAllUsersQuery, GetOneBudgetQuery,
            GetOneBudgetQueryResult, GetOneCategoryQuery, GetOneUserQuery, UpdateBudgetCommand,
            UpdateCategoryCommand, UpdateUserCommand,
        },
    },
};
use std::sync::Arc;

pub struct Service {
    pool: Arc<SqliteConnectionPool>,
    create_budget_command_handler: CreateBudgetService<CreateBudgetRepository>,
    update_budget_command_handler: UpdateBudgetService<UpdateBudgetRepository>,
    get_one_budget_command_handler: GetOneBudgetService<GetOneBudgetRepository>,
    get_all_budgets_command_handler: GetAllBudgetsService<GetAllBudgetsRepository>,
    create_category_command_handler: CreateCategoryService<CreateCategoryRepository>,
    update_category_command_handler: UpdateCategoryService<UpdateCategoryRepository>,
    get_one_category_command_handler: GetOneCategoryService<GetOneCategoryRepository>,
    get_all_categories_command_handler: GetAllCategoriesService<GetAllCategoriesRepository>,
    create_user_command_handler: CreateUserService<CreateUserRepository>,
    update_user_command_handler: UpdateUserService<UpdateUserRepository>,
    get_one_user_command_handler: GetOneUserService<GetOneUserRepository>,
    get_all_users_command_handler: GetAllUsersService<GetAllUsersRepository>,
}

impl Service {
    pub async fn new(db_url: &str) -> Self {
        let pool = Arc::new(SqliteConnectionPool::new(db_url).await.unwrap());
        let create_budget_repository = CreateBudgetRepository::new(pool.clone());
        let update_budget_repository = UpdateBudgetRepository::new(pool.clone());
        let get_one_budget_repository = GetOneBudgetRepository::new(pool.clone());
        let get_all_budgets_repository = GetAllBudgetsRepository::new(pool.clone());
        let create_category_repository = CreateCategoryRepository::new(pool.clone());
        let update_category_repository = UpdateCategoryRepository::new(pool.clone());
        let get_one_category_repository = GetOneCategoryRepository::new(pool.clone());
        let get_all_categories_repository = GetAllCategoriesRepository::new(pool.clone());
        let create_user_repository = CreateUserRepository::new(pool.clone());
        let update_user_repository = UpdateUserRepository::new(pool.clone());
        let get_one_user_repository = GetOneUserRepository::new(pool.clone());
        let get_all_users_repository = GetAllUsersRepository::new(pool.clone());

        return Self {
            pool,
            create_budget_command_handler: CreateBudgetService::new(create_budget_repository),
            update_budget_command_handler: UpdateBudgetService::new(update_budget_repository),
            get_one_budget_command_handler: GetOneBudgetService::new(get_one_budget_repository),
            get_all_budgets_command_handler: GetAllBudgetsService::new(get_all_budgets_repository),
            create_category_command_handler: CreateCategoryService::new(create_category_repository),
            update_category_command_handler: UpdateCategoryService::new(update_category_repository),
            get_one_category_command_handler: GetOneCategoryService::new(
                get_one_category_repository,
            ),
            get_all_categories_command_handler: GetAllCategoriesService::new(
                get_all_categories_repository,
            ),
            create_user_command_handler: CreateUserService::new(create_user_repository),
            update_user_command_handler: UpdateUserService::new(update_user_repository),
            get_one_user_command_handler: GetOneUserService::new(get_one_user_repository),
            get_all_users_command_handler: GetAllUsersService::new(get_all_users_repository),
        };
    }

    pub async fn create_budget(&self, budget: CreateBudgetCommand) {
        self.create_budget_command_handler
            .handle(budget)
            .await
            .expect("Failed to create budget");

        println!("Budget created successfully");
        self.pool.close().await;
    }

    pub async fn update_budget(&self, budget: UpdateBudgetCommand) {
        self.update_budget_command_handler
            .handle(budget)
            .await
            .expect("Failed to create budget");

        println!("Budget created successfully");
        self.pool.close().await;
    }

    pub async fn get_budget(
        &self,
        budget: GetOneBudgetQuery,
    ) -> Result<GetOneBudgetQueryResult, GetOneBudgetError> {
        let result = self.get_one_budget_command_handler.handle(budget).await?;

        self.pool.close().await;

        Ok(result)
    }

    pub async fn get_all_budgets(
        &self,
        budget: GetAllBudgetsQuery,
    ) -> Result<Vec<GetAllBudgetsQueryResult>, GetAllBudgetsError> {
        let result = self.get_all_budgets_command_handler.handle(budget).await?;

        self.pool.close().await;

        Ok(result)
    }

    pub async fn create_category(&self, category: CreateCategoryCommand) {
        self.create_category_command_handler
            .handle(category)
            .await
            .expect("Failed to create category");

        println!("Category created successfully");
        self.pool.close().await;
    }

    pub async fn update_category(&self, category: UpdateCategoryCommand) {
        self.update_category_command_handler
            .handle(category)
            .await
            .expect("Failed to update category");

        println!("Category updated successfully");
        self.pool.close().await;
    }

    pub async fn get_category(&self, query: GetOneCategoryQuery) {
        let result = self
            .get_one_category_command_handler
            .handle(query)
            .await
            .expect("Failed to get category");

        println!("Category: {:?}", result);
        self.pool.close().await;
    }

    pub async fn get_all_categories(&self, query: GetAllCategoriesQuery) {
        let result = self
            .get_all_categories_command_handler
            .handle(query)
            .await
            .expect("Failed to get categories");

        println!("Categories: {:?}", result);
        self.pool.close().await;
    }

    pub async fn create_user(&self, user: CreateUserCommand) {
        self.create_user_command_handler
            .handle(user)
            .await
            .expect("Failed to create user");

        println!("User created successfully");
        self.pool.close().await;
    }

    pub async fn update_user(&self, user: UpdateUserCommand) {
        self.update_user_command_handler
            .handle(user)
            .await
            .expect("Failed to update user");

        println!("User updated successfully");
        self.pool.close().await;
    }

    pub async fn get_user(&self, query: GetOneUserQuery) {
        let result = self
            .get_one_user_command_handler
            .handle(query)
            .await
            .expect("Failed to get user");

        println!("User: {:?}", result);
        self.pool.close().await;
    }

    pub async fn get_all_users(&self, query: GetAllUsersQuery) {
        let result = self
            .get_all_users_command_handler
            .handle(query)
            .await
            .expect("Failed to get users");

        println!("Users: {:?}", result);
        self.pool.close().await;
    }
}
