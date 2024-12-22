use sea_orm_migration::prelude::*;

#[async_std::main]
#[allow(unused_variables, unreachable_code)]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
