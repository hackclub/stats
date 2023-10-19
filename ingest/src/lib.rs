pub mod elastic;
pub mod hcb;
pub mod utils;

use hcb::{
    event::Event, session::Session, transaction::CanonicalTransaction, user::User, HcbModel,
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub async fn index_all(
    client: &elasticsearch::Elasticsearch,
) -> Result<(), Box<dyn std::error::Error>> {
    let multi_progress = MultiProgress::new();
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")?
        .progress_chars("#>-");
    let index_task_count = 4;

    let pb1 = multi_progress.add(ProgressBar::new(index_task_count));
    let pb2 = multi_progress.add(ProgressBar::new(index_task_count));
    let pb3 = multi_progress.add(ProgressBar::new(index_task_count));
    let pb4 = multi_progress.add(ProgressBar::new(index_task_count));

    pb1.set_style(style.clone());
    pb2.set_style(style.clone());
    pb3.set_style(style.clone());
    pb4.set_style(style.clone());

    let pb1 = futures::try_join!(
        Event::idx::<Event>(&client, "organizations", "events", pb1),
        User::idx::<User>(&client, "users", "users", pb2),
        CanonicalTransaction::idx::<CanonicalTransaction>(
            &client,
            "transactions",
            "transactions",
            pb3
        ),
        Session::idx::<Session>(&client, "sessions", "sessions", pb4),
    )?;

    Ok(())
}
