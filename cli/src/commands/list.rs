use std::collections::HashMap;

use futures::TryStreamExt;
use sqlx::PgConnection;

pub(crate) async fn list(conn: &mut PgConnection) -> anyhow::Result<()> {
    let available_extensions = available_extensions(conn).await?;

    for (extension, versions) in available_extensions {
        println!("{extension}");
        for version in versions {
            println!("  {version}");
        }
    }

    Ok(())
}

#[derive(sqlx::FromRow)]
struct ExtensionRow {
    name: String,
    version: String,
}

async fn available_extensions(
    conn: &mut PgConnection,
) -> anyhow::Result<HashMap<String, Vec<String>>> {
    let mut rows = sqlx::query_as::<_, ExtensionRow>(
        "select name, version from pgtle.available_extension_versions()",
    )
    .fetch(conn);

    let mut available_extensions = HashMap::new();
    while let Some(row) = rows.try_next().await? {
        let versions: &mut Vec<String> = available_extensions.entry(row.name).or_default();
        versions.push(row.version);
    }

    Ok(available_extensions)
}
