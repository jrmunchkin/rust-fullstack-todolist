use super::db;
use model::*;

use crate::error::Error;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn start_cron() -> Result<(), Error> {
    let sched = JobScheduler::new().await?;

    sched
        .add(
            Job::new_async("0 */1 * * * *", |_uuid, _l| {
                Box::pin(async {
                    reset_datas().await.unwrap();
                })
            })
            .unwrap(),
        )
        .await?;

    sched.start().await?;

    Ok(())
}

async fn reset_datas() -> Result<(), Error> {
    let db = db::DB::init().await?;

    db.delete_all_todos().await?;

    db.create_todo(&TodoCreateRequest {
        name: String::from("Shopping"),
        is_complete: false,
    })
    .await?;

    db.create_todo(&TodoCreateRequest {
        name: String::from("Feed the dog"),
        is_complete: true,
    })
    .await?;

    db.create_todo(&TodoCreateRequest {
        name: String::from("Change the bedsheets"),
        is_complete: true,
    })
    .await?;

    db.create_todo(&TodoCreateRequest {
        name: String::from("Watch the match"),
        is_complete: false,
    })
    .await?;

    Ok(())
}
