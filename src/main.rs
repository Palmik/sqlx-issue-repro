#[tokio::main]
async fn main() {
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .connect("postgresql://dev:dev_password@localhost:21101/dev")
        .await
        .unwrap();
    {
        let mut tx = db_pool.begin().await.unwrap();
        sqlx::query!("DELETE FROM bar")
            .execute(&mut *tx)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM baz")
            .execute(&mut *tx)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM foo")
            .execute(&mut *tx)
            .await
            .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO foo (id, name) VALUES
                (1, 'foo1'),
                (2, 'foo2'),
                (3, 'foo3')
            "#
        )
        .execute(&mut *tx)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO baz (id, name) VALUES
                (1, 'baz1'),
                (2, 'baz2')
            "#
        )
        .execute(&mut *tx)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO bar (id, name, foo_id, baz_id) VALUES
                (1, 'bar1', 1, 1),
                (2, 'bar2', 2, 2),
                (3, 'bar3', 1, null),
                (4, 'bar4', 2, null)
            "#
        )
        .execute(&mut *tx)
        .await
        .unwrap();

        tx.commit().await.unwrap();
    }

    let foos = sqlx::query!(
        r#"
        SELECT
            foo.id,
            foo.name,
            bar.id AS "bar_id",
            bar.name AS "bar_name",
            baz.id AS "baz_id",
            baz.name AS "baz_name"
        FROM foo
        LEFT JOIN bar ON bar.foo_id = foo.id
        LEFT JOIN baz ON baz.id = bar.baz_id 
        "#
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    for foo in foos {
        println!("{:?}", foo);
    }
}
